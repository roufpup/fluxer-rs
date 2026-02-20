use crate::gateway::data_structure::common::{
    ChannelData, EmojiData, GuildPropertiesData, MemberData, RoleData,
};
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
    pub channels: Vec<ChannelData>,
    pub emojis: Vec<Value>,
    pub id: String,
    pub joined_at: String,
    pub member_count: i64,
    pub members: Vec<MemberData>,
    pub online_count: i64,
    pub presences: Vec<Value>,
    pub properties: GuildPropertiesData,
    pub roles: Vec<RoleData>,
    pub stickers: Vec<Value>,
    pub voice_states: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct GuildEmojisUpdateData {
    pub guild_id: String,
    pub emojis: Vec<EmojiData>,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleCreateData {
    pub guild_id: String,
    pub role: RoleData,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleUpdateBulkData {
    pub guild_id: String,
    pub roles: Vec<RoleData>,
}

#[derive(Deserialize, Debug)]
pub struct GuildRoleDeleteData {
    pub guild_id: String,
    pub role_id: String,
}
