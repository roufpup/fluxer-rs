use std::sync::Arc;

use fluxer_rs::fluxerbot::FluxerBot;

use crate::dispatch::ColorbotDispatchHandler;

pub mod commands;
pub mod dispatch;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let bot: FluxerBot = FluxerBot::init(
        "".to_string(),
        "wss://gateway.fluxer.app?v=1&encoding=json&compress=none".to_string(),
        "https://api.fluxer.app/v1".to_string(),
    )
    .await;

    let bot_arc = Arc::new(bot);
    bot_arc
        .start(ColorbotDispatchHandler {
            bot: bot_arc.clone(),
        })
        .await;
}
