use ezsockets::{Client, Utf8Bytes};
use log::info;

use crate::fluxerbot::FluxerBot;
use crate::gateway::{
    op_handlers::dispatch::DispatchHandlerTrait,
    serde::send_serde::{SendData, SendDataType},
};

pub async fn heartbeat_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    text: Utf8Bytes,
    client_handle: &Client<FluxerBot<T>>,
) {
    info!("-> {}", text);

    let heartbeat = SendData {
        op: 1,
        d: SendDataType::OP1(None),
    };

    let heartbeat_string = serde_json::to_string(&heartbeat).unwrap();
    client_handle.text(&heartbeat_string).unwrap();

    info!("<- {}", heartbeat_string);
}

pub async fn heartbeat_ack_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    text: Utf8Bytes,
) {
    info!("-> {} Heartbeat acknowledged", text);
}
