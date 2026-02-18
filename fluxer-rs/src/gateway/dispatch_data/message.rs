use serde::Deserialize;
use serde_json::Value;

use crate::gateway::dispatch_data::types::{Author, Member};

#[derive(Deserialize)]
pub struct MessageEventData {
    pub attachments: Vec<Value>,
    pub author: Author,
    pub channel_id: String,
    pub channel_type: i64,
    pub content: String,
    pub edited_timestamp: Value,
    pub embeds: Vec<Value>,
    pub flags: i64,
    pub guild_id: String,
    pub id: String,
    pub member: Member,
    pub mention_everyone: bool,
    pub nonce: String,
    pub pinned: bool,
    pub stickers: Vec<Value>,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: i64,
}

#[derive(Deserialize)]
pub struct TypingEventData {
    pub channel_id: String,
    pub guild_id: String,
    pub member: Member,
    pub timestamp: i64,
    pub user_id: String,
}
