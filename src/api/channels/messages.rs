use crate::{
    api::{ApiCall, FluxerApiCallType, FluxerRsError},
    serde::types::message::{Embed, MessageData, MessageReference},
};
use anyhow::Result;
use derive_builder::Builder;
use serde::{Serialize, ser::SerializeMap};

///
///  Fetch message
///

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct FetchMessage {
    pub channel_id: String,
    pub message_id: String,
}

impl ApiCall for FetchMessage {
    type ReturnType = MessageData;

    fn get_req(
        &self,
        req: reqwest::RequestBuilder,
        token: &str,
    ) -> Result<reqwest::RequestBuilder, FluxerRsError> {
        Ok(req.header("Authorization", format!("Bot {token}")))
    }

    fn get_info(&self) -> (String, FluxerApiCallType) {
        (
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Get,
        )
    }

    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        let value = serde_json::from_str::<MessageData>(body)?;
        Ok(value)
    }
}

///
///  Send message
///

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct SendMessage {
    pub channel_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
    #[builder(default)]
    pub message_reference: Option<MessageReference>,
}

impl ApiCall for SendMessage {
    type ReturnType = MessageData;

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
            format!("/channels/{}/messages", self.channel_id),
            FluxerApiCallType::Post,
        )
    }

    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        let value = serde_json::from_str::<MessageData>(body)?;
        Ok(value)
    }
}

impl Serialize for SendMessage {
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

///
///  Edit message
///

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct EditMessage {
    pub channel_id: String,
    pub message_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
}

impl ApiCall for EditMessage {
    type ReturnType = MessageData;

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
            format!("/channels/{}/messages/{}", self.channel_id, self.message_id),
            FluxerApiCallType::Patch,
        )
    }

    fn get_data(&self, body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        let value = serde_json::from_str::<MessageData>(body)?;
        Ok(value)
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
        state.end()
    }
}
