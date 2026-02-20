use crate::api::{ApiCall, FluxerApiCallType, data_structure::role::AddRoleToMember};

impl ApiCall for AddRoleToMember {
    fn get_req(&self, req: minreq::Request, token: String) -> minreq::Request {
        req.with_header("Authorization", format!("Bot {token}"))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!(
                "/guilds/{}/members/{}/roles/{}",
                self.guild_id, self.user_id, self.role_id
            ),
            FluxerApiCallType::Put,
        )
    }
}
