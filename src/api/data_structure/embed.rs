use std::mem::take;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    GifV,
    Article,
    Link,
}

#[derive(Serialize, Clone)]
pub enum EmbedMediaFlags {
    None,
    IsExplicit,
    IsAnimated,
    IsAnimatedAndExplicit,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Builder, Debug)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Builder, Debug)]
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

// Name is a required field even tho the js library makes it nullable

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Builder, Debug)]
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

// Pretty sure the text should be a required field

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Builder, Debug)]
#[builder(try_setter, setter(into))]
pub struct EmbedFooter {
    pub text: Option<String>,
    #[builder(default)]
    pub icon_url: Option<String>,
    #[builder(default)]
    pub proxy_icon_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Default, Builder, Debug)]
#[builder(try_setter, setter(into))]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[builder(default)]
    pub inline: Option<bool>,
}

// WIll implement my own build method for EmbedMedia because i need to manually setup the width and height
// So we don't put Default here as build_fn(skip) makes it so they won't be used

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
#[builder(try_setter, setter(into), build_fn(skip))]
pub struct EmbedMedia {
    pub url: String,
    pub proxy_url: Option<String>,
    pub content_type: Option<String>,
    pub content_hash: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub duration: Option<u64>,
    pub flags: Option<u8>,
}

impl EmbedMediaBuilder {
    pub fn build(&mut self) -> Result<EmbedMedia, EmbedMediaBuilderError> {
        let media_builder = take(self);

        let url = if let Some(result) = media_builder.url {
            result
        } else {
            return Err(EmbedMediaBuilderError::UninitializedField(
                "The url field of the embed media must be used",
            ));
        };

        let res_result: Option<(u32, u32)> =
            if media_builder.width.is_none() || media_builder.height.is_none() {
                let image = minreq::get(&url).send().unwrap();
                let rust_img = image::load_from_memory(image.as_bytes()).unwrap();
                Some((rust_img.width(), rust_img.width()))
            } else {
                None
            };

        Ok(EmbedMedia {
            url,
            proxy_url: media_builder.proxy_url.unwrap_or_default(),
            content_type: media_builder.content_type.unwrap_or_default(),
            content_hash: media_builder.content_hash.unwrap_or_default(),
            width: if let Some(width) = media_builder.width {
                width
            } else {
                Some(res_result.unwrap().0)
            },
            height: if let Some(height) = media_builder.height {
                height
            } else {
                Some(res_result.unwrap().1)
            },
            description: media_builder.description.unwrap_or_default(),
            placeholder: media_builder.placeholder.unwrap_or_default(),
            duration: media_builder.duration.unwrap_or_default(),
            flags: media_builder.flags.unwrap_or(Some(0)),
        })
    }
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
