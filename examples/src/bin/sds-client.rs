use anyhow::Result;
use clap::Parser;
use humantime::format_duration;
use log::{debug, error, info};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use quotesdirectlib::fast::{Message, SecurityDefinition};

use examples::{
    client::{Feeds, SDSClient},
    config::{SDSClientConfig, read_from_file},
    setup_ctrl_c_handler,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file [default: sds-client.yaml])
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Feeds to subscribe to. E.g.: "1-105 !89 107"
    #[arg(short, long, value_name = "FEEDS")]
    feeds: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
            .write_style_or("LOG_STYLE", "always"),
    );

    let args = Args::parse();

    // Load configuration
    let config_path = args
        .config
        .unwrap_or_else(|| PathBuf::from("sds-client.yaml"));
    info!("Loading config file: {}", config_path.display());
    let mut cfg: SDSClientConfig = read_from_file(&config_path)?;
    if let Some(feeds) = args.feeds {
        cfg.feeds = feeds;
    }

    match run(cfg).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            Err(err)
        }
    }
}

async fn run(cfg: SDSClientConfig) -> Result<()> {
    info!("Configuration: {:#?}", cfg);

    let feeds = Feeds::from_str(&cfg.feeds)?;

    let mut sds = SDSClient::new();

    let s = &cfg.sds;
    sds.connect(&s.host, s.port, &s.login, &s.password).await?;
    info!("Subscribing to feeds...");
    sds.subscribe_feeds(feeds).await?;

    let mut subscribed: bool = false;
    let start = SystemTime::now();

    let token = setup_ctrl_c_handler();
    'main: loop {
        let message: Message;
        let is_update: bool;
        tokio::select! {
            _ = token.cancelled() => {
                // sds.logout().await?;
                // continue 'messages;
                break 'main;
            },
            result = sds.read_message() => {
                match result? {
                    None => break 'main,
                    Some((m ,b)) => {
                        message = m;
                        is_update = b;
                    },
                };
            },
        };

        // Process the message
        match message {
            Message::MDSecurityDefinition(m) => {
                if subscribed {
                    if is_update {
                        info!("[UPD] {}", security_definition_as_string(&m));
                    } else {
                        info!("[NEW] {}", security_definition_as_string(&m));
                    }
                } else {
                    debug!("{}", security_definition_as_string(&m));
                }

                if !is_update && !subscribed {
                    if sds.defs_count.is_multiple_of(10000) || sds.is_subscribed() {
                        info!(
                            "Progress {:.1}% ({}/{})",
                            sds.progress(),
                            sds.defs_count,
                            sds.defs_count_total
                        );
                    }
                    if sds.is_subscribed() {
                        info!("Feeds subscribed");
                        let duration = start.elapsed()?;
                        let usec_per_message = duration.as_micros() as f64 / sds.defs_count as f64;
                        info!(
                            "{} messages processed in {} ({usec_per_message:.2}µs/msg)",
                            sds.defs_count,
                            format_duration(Duration::from_secs(duration.as_secs())), // trim to seconds
                        );
                        subscribed = true;
                    }
                }
            }
            Message::MDHeartbeat(_) => {}
            Message::MDLogon(m) => {
                debug!("{m:?}");
            }
            Message::MDLogout(m) => {
                info!("Got logout message");
                debug!("{m:?}");
                break 'main;
            }
            Message::MDSecurityDefinitionRequest(m) => {
                debug!("{m:?}");
            }
            _ => unreachable!(),
        }
    }
    info!("Exiting...");
    Ok(())
}

fn security_definition_as_string(sd: &SecurityDefinition) -> String {
    let symbol = match &sd.cqg_security_name {
        Some(symbol) => symbol.as_str(),
        None => match &sd.symbol {
            Some(symbol) => symbol.as_str(),
            None => sd.security_desc.as_str(),
        },
    };
    format!(
        "MDSecurityDefinition: Symbol: {} ({}), SecurityID: {}, ApplID: {}",
        symbol, sd.security_name, sd.security_id, sd.appl_id
    )
}
