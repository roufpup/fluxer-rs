use fluxer_rs::{error::FluxerRsError, fluxerbot::FluxerBot};

use crate::dispatch::ColorbotDispatchHandler;

pub mod commands;
pub mod dispatch;

#[tokio::main]
async fn main() -> Result<(), FluxerRsError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let bot = FluxerBot::init(
        "<your bot token here>",
        "wss://gateway.fluxer.app?v=1&encoding=json&compress=none",
        "https://api.fluxer.app/v1",
    )?;

    bot.start(ColorbotDispatchHandler {}).await;

    Ok(())
}
