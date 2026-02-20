use log::info;
use serde::{Serialize, ser::SerializeMap};

use crate::api::{ApiCall, FluxerApiCallType, data_structure::message::EditMessage};

impl ApiCall for EditMessage {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        let body = serde_json::to_string(self).unwrap();
        info!("BODY CHECK {body}");
        req.with_body(body)
            .with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Patch,
        )
    }
}

impl Serialize for EditMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("content", &self.content)?;
        if let Some(embed) = self.embeds.clone() {
            state.serialize_entry("embeds", &embed)?;
        };
        if let Some(message_reference) = self.message_reference.clone() {
            state.serialize_entry("message_reference", &message_reference)?;
        }
        state.end()
    }
}
