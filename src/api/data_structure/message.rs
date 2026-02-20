use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::Embed;

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct FetchMessage {
    // Path params
    pub channel_id: String,
    pub message_id: String,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct SendMessage {
    // Path params
    pub channel_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
    #[builder(default)]
    pub message_reference: Option<MessageReference>,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct EditMessage {
    // Path params
    pub channel_id: String,
    pub message_id: String,

    pub content: String,
    #[builder(default)]
    pub embeds: Option<Vec<Embed>>,
    #[builder(default)]
    pub message_reference: Option<MessageReference>,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Builder)]
#[builder(try_setter, setter(into))]
pub struct MessageReference {
    pub message_id: String,
    #[builder(default)]
    pub channel_id: Option<String>,
    #[builder(default)]
    pub guild_id: Option<String>,
}
