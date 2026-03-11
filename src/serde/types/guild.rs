use serde::Deserialize;
use serde_json::Value;

use crate::serde::types::common::{
    ChannelData, EmojiData, GuildPropertiesData, MemberData, RoleData,
};

/// Data returned by the `GUILD_DELETE` dispatch event
#[derive(Deserialize)]
pub struct GuildDeleteData {
    #[serde(rename = "id")]
    pub guild_id: String,
    pub unavailable: bool,
}

/// Data returned by the `GUILD_CREATE` dispatch event
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

/// Data returned by the `GUILD_EMOJIS_UPDATE` dispatch event
#[derive(Deserialize, Debug)]
pub struct GuildEmojisUpdateData {
    pub guild_id: String,
    pub emojis: Vec<EmojiData>,
}

/// Data returned by the `GUILD_ROLE_CREATE` and `GUILD_ROLE_UPDATE` dispatch events
#[derive(Deserialize, Debug)]
pub struct GuildRoleCreateData {
    pub guild_id: String,
    pub role: RoleData,
}

/// Data returned by the `GUILD_ROLE_UPDATE_BULK` dispatch event
#[derive(Deserialize, Debug)]
pub struct GuildRoleUpdateBulkData {
    pub guild_id: String,
    pub roles: Vec<RoleData>,
}

/// Data returned by the `GUILD_ROLE_DELETE` dispatch event
#[derive(Deserialize, Debug)]
pub struct GuildRoleDeleteData {
    pub guild_id: String,
    pub role_id: String,
}
