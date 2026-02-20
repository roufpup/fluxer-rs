pub mod calls;
pub mod data_structure;

use log::info;
use minreq::Response;

pub enum FluxerApiCallType {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

pub trait ApiCall {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request;
    fn get_info(&self) -> (String, FluxerApiCallType);
}

#[derive(Clone, Default)]
pub struct FluxerApiHandler {
    pub token: String,
    pub api_endpoint: String,
}

impl FluxerApiHandler {
    pub fn execute_call<T: ApiCall>(&self, call: T) -> Result<Response, minreq::Error> {
        let (endpoint, call_type) = call.get_info();
        let url = format!("{}{endpoint}", self.api_endpoint);

        let request = match call_type {
            FluxerApiCallType::Get => {
                let req = minreq::get(url);
                call.get_req(req, self.token.to_string())
            }
            FluxerApiCallType::Post => {
                let req = minreq::post(url);
                call.get_req(req, self.token.to_string())
            }
            FluxerApiCallType::Patch => {
                let req = minreq::patch(url);
                call.get_req(req, self.token.to_string())
            }
            FluxerApiCallType::Put => {
                let req = minreq::put(url);
                call.get_req(req, self.token.to_string())
            }
            FluxerApiCallType::Delete => {
                let req = minreq::delete(url);
                call.get_req(req, self.token.to_string())
            }
        };

        info!("REQ CHECK {:?}", request);
        let resp = request.send();
        info!("RESP CHECK: {:?}", resp);
        resp
    }
}
