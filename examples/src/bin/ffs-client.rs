use anyhow::Result;
use clap::Parser;
use log::{debug, error, info};
use std::path::PathBuf;

use fastlib::Decoder;
use quotesdirectlib::{
    fast::{Message, TEMPLATES_XML},
    packets::UDPPacket,
};

use examples::{
    config::{read_from_file, FFSClientConfig},
    network::make_multicast_udp_socket,
    setup_ctrl_c_handler,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file [default: ffs-client.yaml])
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
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
        .unwrap_or_else(|| PathBuf::from("ffs-client.yaml"));
    info!("Loading config file: {}", config_path.display());
    let cfg: FFSClientConfig = read_from_file(&config_path)?;

    match run(cfg).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error: {}", err);
            Err(err)
        }
    }
}

const MAX_DATAGRAM_SIZE: usize = 65507;

async fn run(cfg: FFSClientConfig) -> Result<()> {
    info!("Configuration: {:#?}", cfg);

    let socket = make_multicast_udp_socket(
        &cfg.connection.mcast_group,
        cfg.connection.mcast_port,
        &cfg.interface,
        &cfg.rcvbuf
    ).await?;

    let mut decoder = Decoder::new_from_xml(TEMPLATES_XML)?;
    let token = setup_ctrl_c_handler();

    let mut buffer = [0u8; MAX_DATAGRAM_SIZE];
    'main: loop {
        // Read raw data from socket
        let raw = tokio::select! {
            _ = token.cancelled() => {
                debug!("Got cancellation signal");
                break 'main
            },
            result = socket.recv(&mut buffer) => {
                let n = result?;
                &buffer[..n]
            }
        };

        // Parse UDP packet
        let packet = match UDPPacket::read(raw) {
            Ok(pkt) => pkt,
            Err(err) => {
                error!("Failed to parse UDP packet: {err}");
                continue;
            }
        };

        // Parse FAST message
        let message: Message = match fastlib::from_slice(&mut decoder, &packet.payload) {
            Ok(msg) => msg,
            Err(err) => {
                error!("Failed to parse FAST message: {err}");
                continue;
            }
        };
        info!("{:#?}", message);
    }
    info!("Exiting...");
    Ok(())
}
