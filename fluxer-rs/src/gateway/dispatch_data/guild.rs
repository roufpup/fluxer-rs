use crate::gateway::dispatch_data::types::{Channel, Member, GuildProperties, Role};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct GuildDeleteData {
    #[serde(rename = "id")]
    pub guild_id: String,
    pub unavailable: bool,
}

#[derive(Deserialize)]
pub struct GuildCreateData {
    pub channels: Vec<Channel>,
    pub emojis: Vec<Value>,
    pub id: String,
    pub joined_at: String,
    pub member_count: i64,
    pub members: Vec<Member>,
    pub online_count: i64,
    pub presences: Vec<Value>,
    pub properties: GuildProperties,
    pub roles: Vec<Role>,
    pub stickers: Vec<Value>,
    pub voice_states: Vec<Value>,
}
