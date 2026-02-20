use crate::{
    api::{ApiCall, FluxerApiCallType, data_structure::message::FetchMessage},
    gateway::data_structure::message::MessageEventData,
};

impl FetchMessage {
    pub fn get_resp(body: &str) -> MessageEventData {
        serde_json::from_str::<MessageEventData>(body).unwrap()
    }
}

impl ApiCall for FetchMessage {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        req.with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Get,
        )
    }
}
