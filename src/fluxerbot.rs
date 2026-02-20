use crate::api::FluxerApiHandler;
use crate::gateway::op_handlers::dispatch::{DispatchHandlerTrait, handle_dispatch_events};
use crate::gateway::op_handlers::heartbeat::{heartbeat_ack_handler, heartbeat_handler};
use crate::gateway::op_handlers::identify::auth_handler;
use crate::gateway::serde::deserialize::{ReceiveData, ReceiveDataType};
use async_trait::async_trait;
use ezsockets::{Bytes, Client, ClientConfig, ClientExt, Error, Utf8Bytes, connect};
use log::{error, info};

//TODO: mark some modules as crate only as they are not used by crate users

#[derive(Clone, Default)]
pub struct FluxerBot {
    token: String,
    endpoint: String,
    pub api: FluxerApiHandler,
}

pub struct FluxerWebsocket<T: DispatchHandlerTrait + Send + Sync + 'static> {
    pub dispatch_handler: T,
    pub ws_handle: Client<FluxerWebsocket<T>>,
    bot_arc: FluxerBot,
}

impl FluxerBot {
    pub async fn init(token: String, wss_endpoint: String, api_endpoint: String) -> Self {
        FluxerBot {
            token: token.clone(),
            endpoint: wss_endpoint,
            api: FluxerApiHandler {
                token,
                api_endpoint,
            },
        }
    }

    pub async fn start<T: DispatchHandlerTrait + Send + Sync + 'static>(
        &self,
        dispatch_handler: T,
    ) {
        info!("Init the bot");

        let config: ClientConfig = ClientConfig::new(self.endpoint.as_str());

        info!("Starting websocket");
        let (_, future) = connect(
            |ws_handle| FluxerWebsocket {
                dispatch_handler,
                ws_handle,
                bot_arc: self.clone(),
            },
            config,
        )
        .await;
        let _ = future.await;
    }
}

impl<T> FluxerWebsocket<T> where T: Send + Sync + DispatchHandlerTrait + 'static {}

#[async_trait]
impl<T: DispatchHandlerTrait + Send + Sync + 'static> ClientExt for FluxerWebsocket<T> {
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
                    auth_handler(&self.bot_arc.token, &self.ws_handle).await
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
        auth_handler(&self.bot_arc.token, &self.ws_handle).await;
        Ok(())
    }
}
