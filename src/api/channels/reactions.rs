use anyhow::Result;
use derive_builder::Builder;

use crate::api::{ApiCall, FluxerApiCallType, FluxerRsError};

///
///  Add own reaction
///

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct AddOwnReaction {
    pub channel_id: String,
    pub message_id: String,
    pub emoji: String,
}

impl ApiCall for AddOwnReaction {
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
                "/channels/{}/messages/{}/reactions/{}/@me",
                self.channel_id, self.message_id, self.emoji
            ),
            FluxerApiCallType::Put,
        )
    }

    fn get_data(&self, _body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        Ok(())
    }
}

///
///  Remove all reactions
///

#[derive(Clone, Debug, Builder)]
#[builder(try_setter, setter(into))]
pub struct RemoveAllEmojiReactions {
    pub channel_id: String,
    pub message_id: String,
    pub emoji: String,
}

impl ApiCall for RemoveAllEmojiReactions {
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
                "/channels/{}/messages/{}/reactions/{}",
                self.channel_id, self.message_id, self.emoji
            ),
            FluxerApiCallType::Delete,
        )
    }

    fn get_data(&self, _body: &str) -> Result<Self::ReturnType, FluxerRsError> {
        Ok(())
    }
}
