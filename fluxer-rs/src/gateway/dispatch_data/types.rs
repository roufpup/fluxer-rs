use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Channel {
    pub guild_id: String,
    pub id: String,
    pub name: String,
    pub permission_overwrites: Vec<Value>,
    pub position: i64,
    #[serde(rename = "type")]
    pub channel_type: i64,
    pub last_message_id: Option<String>,
    pub last_pin_timestamp: Option<Value>,
    pub nsfw: Option<bool>,
    pub parent_id: Option<String>,
    pub rate_limit_per_user: Option<i64>,
    pub topic: Option<Value>,
    pub bitrate: Option<i64>,
    pub rtc_region: Option<Value>,
    pub user_limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct Member {
    pub accent_color: Value,
    pub avatar: Value,
    pub banner: Value,
    pub communication_disabled_until: Value,
    pub deaf: bool,
    pub guild_id: Option<String>,
    pub joined_at: String,
    pub mute: bool,
    pub nick: Value,
    pub roles: Vec<String>,
    pub user: Option<User>,
}

#[derive(Deserialize)]
pub struct User {
    pub avatar: Value,
    pub avatar_color: Value,
    pub bot: Option<bool>,
    pub discriminator: String,
    pub flags: i64,
    pub global_name: Value,
    pub id: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct GuildProperties {
    pub afk_channel_id: Value,
    pub afk_timeout: i64,
    pub banner: Value,
    pub banner_height: Value,
    pub banner_width: Value,
    pub default_message_notifications: i64,
    pub disabled_operations: i64,
    pub embed_splash: Value,
    pub embed_splash_height: Value,
    pub embed_splash_width: Value,
    pub explicit_content_filter: i64,
    pub features: Vec<String>,
    pub icon: Value,
    pub id: String,
    pub message_history_cutoff: Value,
    pub mfa_level: i64,
    pub name: String,
    pub nsfw_level: i64,
    pub owner_id: String,
    pub rules_channel_id: Value,
    pub splash: Value,
    pub splash_card_alignment: i64,
    pub splash_height: Value,
    pub splash_width: Value,
    pub system_channel_flags: i64,
    pub system_channel_id: String,
    pub vanity_url_code: Value,
    pub verification_level: i64,
}

#[derive(Deserialize)]
pub struct Role {
    pub color: i64,
    pub hoist: bool,
    pub hoist_position: Value,
    pub id: String,
    pub mentionable: bool,
    pub name: String,
    pub permissions: String,
    pub position: i64,
}

#[derive(Deserialize)]
pub struct Author {
    pub avatar: String,
    pub avatar_color: i64,
    pub discriminator: String,
    pub flags: i64,
    pub global_name: String,
    pub id: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Emoji {
    pub name: String,
}


