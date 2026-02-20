use crate::api::{ApiCall, FluxerApiCallType, data_structure::role::DeleteRole};

impl ApiCall for DeleteRole {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        req.with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/guilds/{}/roles/{}", self.guild_id, self.role_id),
            FluxerApiCallType::Delete,
        )
    }
}
