use anyhow::Result;
use ezsockets::{Client, Utf8Bytes};
use log::debug;

use crate::error::FluxerRsError;
use crate::fluxerbot::FluxerWebsocket;
use crate::{
    gateway::dispatch::DispatchHandlerTrait,
    serde::types::gateway::{SendData, SendDataType},
};

pub async fn heartbeat_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    text: Utf8Bytes,
    client_handle: &Client<FluxerWebsocket<T>>,
) -> Result<(), FluxerRsError> {
    debug!("-> {}", text);

    let heartbeat = SendData {
        op: 1,
        d: SendDataType::OP1(None),
    };

    let heartbeat_string = serde_json::to_string(&heartbeat)?;
    client_handle
        .text(&heartbeat_string)
        .map_err(|err| FluxerRsError::SendError(err.to_string()))?;

    debug!("<- {}", heartbeat_string);

    Ok(())
}

pub async fn heartbeat_ack_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    text: Utf8Bytes,
) {
    debug!("-> {} Heartbeat acknowledged", text);
}
