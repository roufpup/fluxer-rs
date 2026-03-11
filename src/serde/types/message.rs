use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::serde::types::common::{AuthorData, EmojiData, MemberData};

/// Data returned from dispatch events of types `MESSAGE_CREATE` `MESSAGE_UPDATE` and `MESSAGE_DELETE`
#[derive(Deserialize, Debug)]
pub struct MessageData {
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

/// Data for when a user starts typing, returned from dispatch events of type `TYPING_START`
#[derive(Deserialize, Debug)]
pub struct TypingStartData {
    pub channel_id: String,
    pub guild_id: String,
    pub member: MemberData,
    pub timestamp: i64,
    pub user_id: String,
}

/// Data for a message reaction, returned from dispatch events of type `MESSAGE_REACTION_ADD`, `MESSAGE_REACTION_REMOVE` and `MESSAGE_REACTION_REMOVE_EMOJI`
#[derive(Deserialize, Debug)]
pub struct MessageReactionData {
    pub channel_id: String,
    pub emoji: EmojiData,
    pub guild_id: String,
    pub member: Option<MemberData>,
    pub message_id: String,
    pub user_id: Option<String>,
}

/// Data for a message reference used when replying to another message, it's used as a part of [`SendMessage`](crate::api::channels::messages) api call.
#[skip_serializing_none]
#[derive(Clone, Debug, Builder, Serialize)]
#[builder(try_setter, setter(into))]
pub struct MessageReference {
    pub message_id: String,
    #[builder(default)]
    pub channel_id: Option<String>,
    #[builder(default)]
    pub guild_id: Option<String>,
}

/// Data for an embed received or sent as part of a message. <br>
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into), default)]
pub struct Embed {
    #[serde(rename = "type")]
    pub embed_type: Option<EmbedType>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub color: Option<u32>,
    pub description: Option<String>,
    pub author: Option<EmbedAuthor>,
    pub image: Option<EmbedMedia>,
    pub thumbnail: Option<EmbedMedia>,
    pub footer: Option<EmbedFooter>,
    pub fields: Option<Vec<EmbedField>>,
    pub provider: Option<EmbedAuthor>,
    pub video: Option<EmbedMedia>,
    pub audio: Option<EmbedMedia>,
    pub nsfw: Option<bool>,
    pub children: Option<Vec<EmbedChild>>,
}

/// The same type as [`Embed`] with the only difference that it cannot have any embed children and it's used only in [`Embed`]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into), default)]
pub struct EmbedChild {
    #[serde(rename = "type")]
    pub embed_type: Option<EmbedType>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub color: Option<u32>,
    pub description: Option<String>,
    pub author: Option<EmbedAuthor>,
    pub image: Option<EmbedMedia>,
    pub thumbnail: Option<EmbedMedia>,
    pub footer: Option<EmbedFooter>,
    pub fields: Option<Vec<EmbedField>>,
    pub provider: Option<EmbedAuthor>,
    pub video: Option<EmbedMedia>,
    pub audio: Option<EmbedMedia>,
    pub nsfw: Option<bool>,
}

/// Data for an author of an embed used in [`Embed`] <br>
/// The `name` field is required
#[skip_serializing_none]
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into))]
pub struct EmbedAuthor {
    pub name: String,
    #[builder(default)]
    pub url: Option<String>,
    #[builder(default)]
    pub icon_url: Option<String>,
    #[builder(default)]
    pub proxy_icon_url: Option<String>,
}

/// Data for a footer of an embed used in [`Embed`] <br>
/// The `text` field is required
#[skip_serializing_none]
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into))]
pub struct EmbedFooter {
    pub text: Option<String>,
    #[builder(default)]
    pub icon_url: Option<String>,
    #[builder(default)]
    pub proxy_icon_url: Option<String>,
}

/// Data for a footer of an embed used in [`Embed`] <br>
/// The `name` and `value` fields are required
#[skip_serializing_none]
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into))]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[builder(default)]
    pub inline: Option<bool>,
}

/// The type of an embed used in [`Embed`] <br>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    GifV,
    Article,
    Link,
}

/// A media flag to mark an embed media as explicit/animated used in [`Embed`] <br>
#[derive(Clone, Debug, Serialize)]
pub enum EmbedMediaFlags {
    None,
    IsExplicit,
    IsAnimated,
    IsAnimatedAndExplicit,
}

/// Data for an emded media used in [`Embed`] <br>
/// The `url` field is required
#[skip_serializing_none]
#[derive(Clone, Debug, Builder, Serialize, Deserialize)]
#[builder(try_setter, setter(into))]
pub struct EmbedMedia {
    pub url: String,
    #[builder(default)]
    pub proxy_url: Option<String>,
    #[builder(default)]
    pub content_type: Option<String>,
    #[builder(default)]
    pub content_hash: Option<String>,
    #[builder(default)]
    pub width: Option<u32>,
    #[builder(default)]
    pub height: Option<u32>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub placeholder: Option<String>,
    #[builder(default)]
    pub duration: Option<u64>,
    #[builder(default)]
    pub flags: Option<u8>,
}

impl From<EmbedMediaFlags> for Option<u8> {
    fn from(value: EmbedMediaFlags) -> Self {
        let result = match value {
            EmbedMediaFlags::None => 0,
            EmbedMediaFlags::IsExplicit => 16,
            EmbedMediaFlags::IsAnimated => 32,
            EmbedMediaFlags::IsAnimatedAndExplicit => 48,
        };
        Some(result)
    }
}
