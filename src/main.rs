use fluxer_rs::{FluxerBot, gateway::op_handlers::dispatch::DispatchHandler};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    FluxerBot::start(
        "".to_string(),
        "wss://gateway.fluxer.app?v=1&encoding=json&compress=none".to_string(),
        DispatchHandler,
    )
    .await;
}
