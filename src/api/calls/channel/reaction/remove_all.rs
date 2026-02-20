use crate::api::{ApiCall, FluxerApiCallType, data_structure::reaction::RemoveAllReaction};

impl ApiCall for RemoveAllReaction {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        req.with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!(
                "/channels/{}/messages/{}/reactions/{}",
                self.channel_id, self.message_id, self.emoji
            ),
            FluxerApiCallType::Delete,
        )
    }
}
