use ezsockets::Client;
use crate::fluxerbot::FluxerBot;
use crate::gateway::op_handlers::dispatch::DispatchHandlerTrait;
use crate::gateway::serde::send_serde::{OP2DProps, SendData, SendDataType, OP2D};

pub async fn auth_handler<T: DispatchHandlerTrait + Send + Sync + 'static> (token: String, client_handle: &Client<FluxerBot<T>>){
    let auth_string = serde_json::to_string(&SendData {
        d: SendDataType::OP2(OP2D {
            token,
            properties: OP2DProps {
                os: "Linux".to_string(),
                browser: "Fluxer-rs".to_string(),
                device: "x64".to_string(),
            },
        }),
        op: 2,
    }).unwrap();

    match client_handle.text(auth_string) {
        Ok(_) => {}
        Err(err) => {
            panic!("{err}")
        }
    };
}