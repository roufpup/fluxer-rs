use serde::Deserialize;
use serde_json::Value;

use crate::{
    Embed,
    gateway::data_structure::common::{AuthorData, EmojiData, MemberData},
};

#[derive(Deserialize, Debug)]
pub struct MessageEventData {
    pub attachments: Option<Vec<Value>>,
    pub author: Option<AuthorData>,
    pub channel_id: String,
    pub channel_type: Option<i64>,
    pub content: String,
    pub edited_timestamp: Option<Value>,
    pub embeds: Option<Vec<Embed>>,
    pub flags: Option<i64>,
    pub guild_id: Option<String>,
    pub id: String,
    pub member: Option<MemberData>,
    pub mention_everyone: Option<bool>,
    pub nonce: Option<String>,
    pub pinned: Option<bool>,
    pub stickers: Option<Vec<Value>>,
    pub timestamp: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct TypingEventData {
    pub channel_id: String,
    pub guild_id: String,
    pub member: MemberData,
    pub timestamp: i64,
    pub user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct MessageReactData {
    pub channel_id: String,
    pub emoji: EmojiData,
    pub guild_id: String,
    pub member: Option<MemberData>,
    pub message_id: String,
    pub user_id: Option<String>,
}
