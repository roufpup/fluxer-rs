use anyhow::Result;
use derive_builder::Builder;
use serde::{Serialize, ser::SerializeMap};

use crate::{
    api::{ApiCall, FluxerApiCallType, FluxerRsError},
    serde::types::common::RoleData,
};

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct AddRoleToMember {
    pub guild_id: String,
    pub user_id: String,
    pub role_id: String,
}

impl ApiCall for AddRoleToMember {
    type ReturnType = ();

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        Ok(req.header("Authorization", format!("Bot {token}")))
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

    fn get_data(&self, _body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        Ok(())
    }
}

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct RemoveRoleFromMember {
    // Path params
    pub guild_id: String,
    pub user_id: String,
    pub role_id: String,
}

impl ApiCall for RemoveRoleFromMember {
    type ReturnType = ();

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        Ok(req.header("Authorization", format!("Bot {token}")))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!(
                "/guilds/{}/members/{}/roles/{}",
                self.guild_id, self.user_id, self.role_id
            ),
            FluxerApiCallType::Delete,
        )
    }

    fn get_data(&self, _body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        Ok(())
    }
}

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct CreateRole {
    // Path params
    pub guild_id: String,

    pub name: String,
    pub color: u32,
    pub permission: String,
}

impl ApiCall for CreateRole {
    type ReturnType = RoleData;

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        let value = serde_json::to_string(self)?;

        Ok(req
            .body(value)
            .header("Authorization", format!("Bot {token}")))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/guilds/{}/roles", self.guild_id),
            FluxerApiCallType::Post,
        )
    }

    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        let value = serde_json::from_str::<RoleData>(body)?;
        Ok(value)
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

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct DeleteRole {
    // Path params
    pub guild_id: String,
    pub role_id: String,
}

impl ApiCall for DeleteRole {
    type ReturnType = ();

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        Ok(req.header("Authorization", format!("Bot {token}")))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/guilds/{}/roles/{}", self.guild_id, self.role_id),
            FluxerApiCallType::Delete,
        )
    }

    fn get_data(&self, _body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        Ok(())
    }
}
