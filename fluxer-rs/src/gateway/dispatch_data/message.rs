use serde::Deserialize;
use serde_json::Value;

use crate::gateway::dispatch_data::types::{Author, Emoji, Member};

#[derive(Deserialize)]
pub struct MessageEventData {
    pub attachments: Option<Vec<Value>>,
    pub author: Option<Author>,
    pub channel_id: String,
    pub channel_type: Option<i64>,
    pub content: String,
    pub edited_timestamp: Option<Value>,
    pub embeds: Option<Vec<Value>>,
    pub flags: Option<i64>,
    pub guild_id: String,
    pub id: String,
    pub member: Member,
    pub mention_everyone: Option<bool>,
    pub nonce: Option<String>,
    pub pinned: Option<bool>,
    pub stickers: Option<Vec<Value>>,
    pub timestamp: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<i64>,
}

#[derive(Deserialize)]
pub struct TypingEventData {
    pub channel_id: String,
    pub guild_id: String,
    pub member: Member,
    pub timestamp: i64,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct MessageReactData {
    pub channel_id: String,
    pub emoji: Emoji,
    pub guild_id: String,
    pub member: Member,
    pub message_id: String,
    pub user_id: String,
}