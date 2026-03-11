use crate::error::FluxerRsError;
use crate::fluxerbot::FluxerWebsocket;
use crate::gateway::dispatch::DispatchHandlerTrait;
use crate::serde::types::gateway::{OP2D, OP2DProps, SendData, SendDataType};
use anyhow::Result;
use ezsockets::Client;
use log::error;

pub async fn auth_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    token: &str,
    client_handle: &Client<FluxerWebsocket<T>>,
) -> Result<(), FluxerRsError> {
    let auth_string = serde_json::to_string(&SendData {
        d: SendDataType::OP2(OP2D {
            token: token.to_string(),
            properties: OP2DProps {
                os: "Linux".to_string(),
                browser: "Fluxer-rs".to_string(),
                device: "x64".to_string(),
            },
        }),
        op: 2,
    })?;

    match client_handle.text(auth_string) {
        Ok(_) => {}
        Err(err) => {
            error!("{err}")
        }
    };

    Ok(())
}
