use log::info;
use serde::{Serialize, ser::SerializeMap};

use crate::api::{ApiCall, FluxerApiCallType, data_structure::role::CreateRole};

impl ApiCall for CreateRole {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        let body = serde_json::to_string(self).unwrap();
        info!("BODY CHECK {body}");
        req.with_body(body)
            .with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/guilds/{}/roles", self.guild_id),
            FluxerApiCallType::Post,
        )
    }
}

impl Serialize for CreateRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("name", &self.name)?;
        state.serialize_entry("color", &self.color)?;
        state.serialize_entry("permissions", &self.permission)?;
        state.end()
    }
}
