use crate::api::{ApiCall, FluxerApiCallType, data_structure::reaction::AddOwnReaciton};

impl ApiCall for AddOwnReaciton {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        req.with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                self.channel_id, self.message_id, self.emoji
            ),
            FluxerApiCallType::Put,
        )
    }
}
