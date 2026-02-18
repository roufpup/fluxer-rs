use ezsockets::{Client, Utf8Bytes};
use log::info;

use crate::{
    FluxerBot,
    gateway::{
        op_handlers::dispatch::DispatchHandlerTrait,
        serde::send_serde::{OP2D, OP2DProps, SendData, SendDataType},
    },
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
    token: String,
    heartbeat_ack: bool,
    client_handle: &Client<FluxerBot<T>>,
) -> bool {
    info!("-> {} Heartbeat acknowledged", text);
    if !heartbeat_ack {
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
        })
        .unwrap();

        match client_handle.text(auth_string) {
            Ok(_) => {}
            Err(err) => {
                panic!("{err}")
            }
        };
        return true;
    }
    false
}
