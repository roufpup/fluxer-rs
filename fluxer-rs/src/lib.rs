use async_trait::async_trait;
use ezsockets::{Bytes, Client, ClientConfig, ClientExt, Utf8Bytes, connect};
use log::{error, info};

use crate::gateway::{
    op_handlers::{
        dispatch::{DispatchHandlerTrait, handle_dispatch_events},
        heartbeat::{heartbeat_ack_handler, heartbeat_handler},
    },
    serde::receive_serde::{ReceiveData, ReceiveDataType},
};

pub mod gateway;

pub struct FluxerBot<T: DispatchHandlerTrait + Send + Sync + 'static> {
    pub token: String,
    pub endpoint: String,
    pub dispatch_handler: T,
    pub ws_handle: Client<FluxerBot<T>>,
    pub heartbeat_ack: bool,
}

impl<T: DispatchHandlerTrait + Send + Sync + 'static> FluxerBot<T> {
    pub async fn start(token: String, endpoint: String, dispatch_handler: T) {
        info!("Init the bot");

        let config: ClientConfig = ClientConfig::new(endpoint.as_str());

        info!("Starting websocket");
        let (_, future) = connect(
            |ws_handle| FluxerBot {
                ws_handle,
                token,
                heartbeat_ack: false,
                dispatch_handler,
                endpoint,
            },
            config,
        )
        .await;
        let _ = future.await;
    }
}

impl<T> FluxerBot<T> where T: Send + Sync + DispatchHandlerTrait + 'static {}

#[async_trait]
impl<T: DispatchHandlerTrait + Send + Sync + 'static> ClientExt for FluxerBot<T> {
    type Call = ();

    async fn on_text(&mut self, text: Utf8Bytes) -> Result<(), ezsockets::Error> {
        let result: ReceiveData = match serde_json::from_slice(text.as_bytes()) {
            Ok(value) => value,
            Err(err) => {
                error!("Unhandled behavior: {err}");
                error!("{}", text);
                panic!()
            }
        };

        match result.d {
            ReceiveDataType::OP10(_data) => heartbeat_handler(text, &__self.ws_handle).await,
            ReceiveDataType::OP11 => {
                //TODO: Implement the check whether this is the first heartbeat a bit better
                self.heartbeat_ack = heartbeat_ack_handler(
                    text,
                    __self.token.clone(),
                    __self.heartbeat_ack,
                    &__self.ws_handle,
                )
                .await
            }
            ReceiveDataType::OP1(_op1_d) => heartbeat_handler(text, &__self.ws_handle).await,
            ReceiveDataType::OP0(dispatch_event) => {
                handle_dispatch_events(dispatch_event, &self.dispatch_handler).await
            }
        }
        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Bytes) -> Result<(), ezsockets::Error> {
        Ok(())
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}
