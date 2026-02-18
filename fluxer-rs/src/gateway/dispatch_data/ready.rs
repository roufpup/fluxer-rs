use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct ReadyData {
    pub country_code: String,
    pub favorite_memes: Vec<Value>,
    pub guilds: Vec<Value>,
    pub notes: Notes,
    pub pinned_dms: Vec<Value>,
    pub presences: Vec<Value>,
    pub private_channels: Vec<Value>,
    pub read_states: Vec<Value>,
    pub relationships: Vec<Value>,
    pub rtc_regions: Vec<RtcRegion>,
    pub session_id: String,
    pub sessions: Vec<Session>,
    pub user: LoggedInUser,
    pub user_guild_settings: Vec<Value>,
    pub user_settings: Value,
    pub users: Vec<Value>,
    pub version: i64,
}

#[derive(Deserialize)]
pub struct Session {
    pub afk: bool,
    pub mobile: bool,
    pub session_id: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct LoggedInUser {
    pub accent_color: Value,
    pub acls: Vec<Value>,
    pub authenticator_types: Vec<i64>,
    pub avatar: Value,
    pub avatar_color: Value,
    pub banner: Value,
    pub banner_color: Value,
    pub bio: Value,
    pub bot: bool,
    pub discriminator: String,
    pub email: Value,
    pub email_bounced: bool,
    pub flags: i64,
    pub global_name: Value,
    pub has_dismissed_premium_onboarding: bool,
    pub has_ever_purchased: bool,
    pub has_unread_gift_inventory: bool,
    pub id: String,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub nsfw_allowed: bool,
    pub password_last_changed_at: Value,
    pub pending_bulk_message_deletion: Value,
    pub phone: Value,
    pub premium_badge_hidden: bool,
    pub premium_badge_masked: bool,
    pub premium_badge_sequence_hidden: bool,
    pub premium_badge_timestamp_hidden: bool,
    pub premium_billing_cycle: Value,
    pub premium_enabled_override: bool,
    pub premium_lifetime_sequence: Value,
    pub premium_purchase_disabled: bool,
    pub premium_since: Value,
    pub premium_type: i64,
    pub premium_until: Value,
    pub premium_will_cancel: bool,
    pub pronouns: Value,
    pub required_actions: Value,
    pub traits: Vec<Value>,
    pub unread_gift_inventory_count: i64,
    pub used_mobile_client: bool,
    pub username: String,
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct RtcRegion {
    pub emoji: String,
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Notes {}
