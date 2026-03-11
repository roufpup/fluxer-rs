use crate::api::{FluxerApiHandler, FluxerApiHandlerBuilder};
use crate::error::{ApiHandlerError, FluxerRsError};
use crate::gateway::dispatch::{DispatchHandlerTrait, handle_dispatch_events};
use crate::gateway::heartbeat::{heartbeat_ack_handler, heartbeat_handler};
use crate::gateway::identify::auth_handler;
use crate::serde::types::gateway::{ReceiveData, ReceiveDataType};
use anyhow::Result;
use async_trait::async_trait;
use ezsockets::{Bytes, Client, ClientConfig, ClientExt, Error, Utf8Bytes, connect};
use log::{debug, error, info};

//TODO: mark some modules as crate only as they are not used by crate users

#[derive(Clone, Default)]
pub struct FluxerBot {
    wss_endpoint: String,
    pub api: FluxerApiHandler,
}

pub struct FluxerWebsocket<T: DispatchHandlerTrait + Send + Sync + 'static> {
    pub dispatch_handler: T,
    pub ws_handle: Client<FluxerWebsocket<T>>,
    bot: FluxerBot,
}

impl FluxerBot {
    pub fn init(
        token: impl Into<String>,
        wss_endpoint: impl Into<String>,
        api_endpoint: impl Into<String>,
    ) -> Result<Self, FluxerRsError> {
        let api = FluxerApiHandlerBuilder::default()
            .api_endpoint(api_endpoint)
            .token(token)
            .http_client(reqwest::Client::new())
            .build()
            .map_err(ApiHandlerError::from)?;
        let wss_endpoint = wss_endpoint.into();

        Ok(FluxerBot { wss_endpoint, api })
    }

    pub async fn start<T: DispatchHandlerTrait + Send + Sync + 'static>(self, dispatch_handler: T) {
        info!("Initializing the bot");

        let config: ClientConfig = ClientConfig::new(self.wss_endpoint.as_str());

        info!("Starting websocket");
        let (_, future) = connect(
            |ws_handle| FluxerWebsocket {
                dispatch_handler,
                ws_handle,
                bot: self,
            },
            config,
        )
        .await;
        match future.await {
            Ok(_) => {
                info!("Executing future")
            }
            Err(err) => {
                log::error!("{err}");
            }
        }
    }
}

impl<T> FluxerWebsocket<T> where T: Send + Sync + DispatchHandlerTrait + 'static {}

#[async_trait]
impl<T: DispatchHandlerTrait + Send + Sync + 'static> ClientExt for FluxerWebsocket<T> {
    type Call = ();

    async fn on_text(&mut self, text: Utf8Bytes) -> Result<(), Error> {
        let result: Option<ReceiveData> = match serde_json::from_slice(text.as_bytes()) {
            Ok(value) => value,
            Err(err) => {
                error!("Unhandled behavior: {err}");
                error!("{}", text);
                None
            }
        };

        if let Some(result) = result {
            match result.d {
                ReceiveDataType::OP0(dispatch_event) => {
                    handle_dispatch_events(dispatch_event, &self.dispatch_handler, &self.bot.api)
                        .await?
                }
                ReceiveDataType::OP1(_op1_d) => heartbeat_handler(text, &self.ws_handle).await?,
                ReceiveDataType::OP9(op9_d) => {
                    if !op9_d {
                        debug!("-> {} Connection Invalid, Reauthenticating", text);

                        auth_handler(&self.bot.api.token, &self.ws_handle).await?
                    } else {
                        //TODO: Implement proper session resume
                        debug!("-> {} Connection Invalid, Resuming", text);
                        panic!()
                    }
                }
                ReceiveDataType::OP10(_data) => heartbeat_handler(text, &self.ws_handle).await?,
                ReceiveDataType::OP11 => heartbeat_ack_handler::<T>(text).await,
            }
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
        auth_handler(&self.bot.api.token, &self.ws_handle).await?;
        Ok(())
    }
}
