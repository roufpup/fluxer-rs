use crate::gateway::op_handlers::dispatch::{DispatchHandlerTrait, handle_dispatch_events};
use crate::gateway::op_handlers::heartbeat::{heartbeat_ack_handler, heartbeat_handler};
use crate::gateway::op_handlers::identify::auth_handler;
use crate::gateway::serde::receive_serde::{ReceiveData, ReceiveDataType};
use async_trait::async_trait;
use ezsockets::{Bytes, Client, ClientConfig, ClientExt, Error, Utf8Bytes, connect};
use log::{error, info};

pub struct FluxerBot<T: DispatchHandlerTrait + Send + Sync + 'static> {
    pub token: String,
    pub endpoint: String,
    pub dispatch_handler: T,
    pub ws_handle: Client<FluxerBot<T>>,
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

    async fn on_text(&mut self, text: Utf8Bytes) -> Result<(), Error> {
        let result: ReceiveData = match serde_json::from_slice(text.as_bytes()) {
            Ok(value) => value,
            Err(err) => {
                error!("Unhandled behavior: {err}");
                error!("{}", text);
                panic!()
            }
        };

        match result.d {
            ReceiveDataType::OP0(dispatch_event) => {
                handle_dispatch_events(dispatch_event, &self.dispatch_handler).await
            }
            ReceiveDataType::OP1(_op1_d) => heartbeat_handler(text, &self.ws_handle).await,
            ReceiveDataType::OP9(op9_d) => {
                if !op9_d {
                    info!("-> {} Connection Invalid, Reauthenticating", text);
                    auth_handler(self.token.clone(), &self.ws_handle).await
                } else {
                    //TODO: Implement session resume
                    info!("-> {} Connection Invalid, Resuming", text);
                    panic!()
                }
            }
            ReceiveDataType::OP10(_data) => heartbeat_handler(text, &self.ws_handle).await,
            ReceiveDataType::OP11 => heartbeat_ack_handler::<T>(text).await,
        }
        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Bytes) -> Result<(), Error> {
        Ok(())
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), Error> {
        Ok(())
    }

    async fn on_connect(&mut self) -> Result<(), Error> {
        auth_handler(self.token.clone(), &self.ws_handle).await;
        Ok(())
    }
}
