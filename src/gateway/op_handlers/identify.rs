use crate::fluxerbot::FluxerWebsocket;
use crate::gateway::op_handlers::dispatch::DispatchHandlerTrait;
use crate::gateway::serde::serialize::{OP2D, OP2DProps, SendData, SendDataType};
use ezsockets::Client;

pub async fn auth_handler<T: DispatchHandlerTrait + Send + Sync + 'static>(
    token: &str,
    client_handle: &Client<FluxerWebsocket<T>>,
) {
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
    })
    .unwrap();

    match client_handle.text(auth_string) {
        Ok(_) => {}
        Err(err) => {
            panic!("{err}")
        }
    };
}
