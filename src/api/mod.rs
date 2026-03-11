pub mod channels;
pub mod common;
pub mod guilds;

use anyhow::Result;
use derive_builder::Builder;
use log::debug;

use crate::error::FluxerRsError;

pub enum FluxerApiCallType {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

/// Trait to use when implementing a new API call 
pub trait ApiCall {
    type ReturnType;

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError>;
    fn get_info(&self) -> (String, FluxerApiCallType);
    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError>;
}

#[derive(Clone, Default, Builder)]
#[builder(try_setter, setter(into))]
pub struct FluxerApiHandler {
    pub(crate) token: String,
    pub(crate) api_endpoint: String,
    pub(crate) http_client: reqwest::Client,
}

/// Implementation where all the api calls get called
impl FluxerApiHandler {
    async fn call_api<T: ApiCall>(&self, call: &T) -> Result<reqwest::Response, FluxerRsError> {
        let (endpoint, call_type) = call.get_info();
        let url = format!("{}{endpoint}", self.api_endpoint);

        let request = match call_type {
            FluxerApiCallType::Get => {
                let req = self.http_client.get(url);
                call.get_req(req, &self.token)?
            }
            FluxerApiCallType::Post => {
                let req = self.http_client.post(url);
                call.get_req(req, &self.token)?
            }
            FluxerApiCallType::Patch => {
                let req = self.http_client.patch(url);
                call.get_req(req, &self.token)?
            }
            FluxerApiCallType::Put => {
                let req = self.http_client.put(url);
                call.get_req(req, &self.token)?
            }
            FluxerApiCallType::Delete => {
                let req = self.http_client.delete(url);
                call.get_req(req, &self.token)?
            }
        };

        let resp = request.send().await?;
        debug!("HTTPS RESPONSE: {:?}", resp);
        Ok(resp)
    }

    pub async fn execute_call<T: ApiCall>(&self, call: T) -> Result<T::ReturnType, FluxerRsError> {
        let resp = self.call_api(&call).await?;
        let text = &resp.text().await?;
        debug!("HTTPS RESPONSE CONTENT: {}", text);
        call.get_data(text)
    }

    pub async fn execute_call_resp<T: ApiCall>(
        &self,
        call: T,
    ) -> Result<reqwest::Response, FluxerRsError> {
        self.call_api(&call).await
    }
}
