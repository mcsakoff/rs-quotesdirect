use log::info;
use tokio::signal::unix::SignalKind;
use tokio_util::sync::CancellationToken;

pub mod client;
pub mod config;
pub mod network;

pub fn setup_ctrl_c_handler() -> CancellationToken {
    let token = CancellationToken::new();
    let token_out = token.clone();
    tokio::spawn(async move {
        let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Got SIGINT");
                token.cancel();
            },
            _ = sigterm.recv() => {
                info!("Got SIGTERM");
                token.cancel();
            },
            _ = token.cancelled() => {
                info!("Got cancellation signal");
            },
        }
    });
    token_out
}
