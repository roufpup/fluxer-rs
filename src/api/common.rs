use crate::{
    api::{
        FluxerApiHandler,
        channels::{
            messages::{EditMessageBuilder, FetchMessageBuilder, SendMessageBuilder},
            reactions::{AddOwnReactionBuilder, RemoveAllEmojiReactionsBuilder},
        },
        guilds::roles::{
            AddRoleToMemberBuilder, CreateRoleBuilder, DeleteRoleBuilder,
            RemoveRoleFromMemberBuilder,
        },
    },
    error::{ApiHandlerError, FluxerRsError},
    serde::types::{
        common::RoleData,
        message::{Embed, MessageData, MessageReferenceBuilder},
    },
};

//
// Message helper functions
//

/// Get a message by it's id
/// Required fields: `channel_id` and `message_id` of the message that will be fetched
pub async fn fetch_message(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
) -> Result<MessageData, FluxerRsError> {
    let call = FetchMessageBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

/// Send a message
/// Required fields: `channel_id` and `content` of the message that will be sent
pub async fn send_message(
    api: &FluxerApiHandler,
    channel_id: &str,
    content: &str,
) -> Result<MessageData, FluxerRsError> {
    let call = SendMessageBuilder::default()
        .channel_id(channel_id)
        .content(content)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

/// Send a message reply
/// Required fields: `channel_id` and `message_id` of the original message and `content` of the response
pub async fn send_reply(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    content: &str,
) -> Result<MessageData, FluxerRsError> {
    let call = SendMessageBuilder::default()
        .channel_id(channel_id)
        .content(content)
        .message_reference(
            MessageReferenceBuilder::default()
                .message_id(message_id)
                .build()
                .map_err(ApiHandlerError::from)?,
        )
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

/// Edit a message
/// Required fields: `channel_id` and `message_id` of the original message and `content` of the edited message
pub async fn edit_message(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    content: &str,
) -> Result<MessageData, FluxerRsError> {
    let call = EditMessageBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .content(content)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

/// Edit a message that has embeds
/// Required fields: `channel_id` and `message_id` of the original message, `content` and `embeds` of the edited message
pub async fn edit_message_with_embeds(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    content: &str,
    embeds: Vec<Embed>,
) -> Result<MessageData, FluxerRsError> {
    let call = EditMessageBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .content(content)
        .embeds(embeds)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

//
// Role helper functions
//

/// Give a role to a member
/// Required fields: `guild_id`, `role_id` and `user_id`
pub async fn give_role(
    api: &FluxerApiHandler,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
) -> Result<(), FluxerRsError> {
    let call = AddRoleToMemberBuilder::default()
        .guild_id(guild_id)
        .role_id(role_id)
        .user_id(user_id)
        .build()
        .map_err(ApiHandlerError::from)?;

    api.execute_call(call).await?;

    Ok(())
}

/// Remove a role from a user
/// Required fields: `guild_id`, `role_id` and `user_id`
pub async fn remove_role(
    api: &FluxerApiHandler,
    guild_id: &str,
    role_id: &str,
    user_id: &str,
) -> Result<(), FluxerRsError> {
    let call = RemoveRoleFromMemberBuilder::default()
        .guild_id(guild_id)
        .role_id(role_id)
        .user_id(user_id)
        .build()
        .map_err(ApiHandlerError::from)?;

    api.execute_call(call).await?;

    Ok(())
}

/// Create a new role
/// Required fields: `guild_id`, `name`, `permission` and `color`
pub async fn create_role(
    api: &FluxerApiHandler,
    guild_id: &str,
    name: &str,
    permission: &str,
    color: &str,
) -> Result<RoleData, FluxerRsError> {
    let call = CreateRoleBuilder::default()
        .guild_id(guild_id)
        .name(name)
        .permission(permission)
        .color(u32::from_str_radix(color, 16).map_err(FluxerRsError::from)?)
        .build()
        .map_err(ApiHandlerError::from)?;

    let result = api.execute_call(call).await?;

    Ok(result)
}

/// Delete an existing role
/// Required fields: `guild_id` and `role_id`
pub async fn delete_role(
    api: &FluxerApiHandler,
    guild_id: &str,
    role_id: &str,
) -> Result<(), FluxerRsError> {
    let call = DeleteRoleBuilder::default()
        .guild_id(guild_id)
        .role_id(role_id)
        .build()
        .map_err(ApiHandlerError::from)?;

    api.execute_call(call).await?;

    Ok(())
}

//
// Reaction helper functions
//

/// Add the bot's reaction to a message
/// Required fields: `channel_id`, `message_id` and `emoji`
pub async fn react(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<(), FluxerRsError> {
    let emoji = if emoji.starts_with("<") {
        emoji.trim_start_matches('<').trim_end_matches('>')
    } else {
        emoji
    };

    let call = AddOwnReactionBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .emoji(emoji)
        .build()
        .map_err(ApiHandlerError::from)?;

    api.execute_call(call).await?;

    Ok(())
}

/// Remove all emoji reactions from a given message
/// Required fields: `channel_id`, `message_id` and `emoji`
pub async fn remove_all_emoji_reactions(
    api: &FluxerApiHandler,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<(), FluxerRsError> {
    let call = RemoveAllEmojiReactionsBuilder::default()
        .channel_id(channel_id)
        .message_id(message_id)
        .emoji(emoji.trim_matches('<').trim_matches('>'))
        .build()
        .map_err(ApiHandlerError::from)?;

    api.execute_call(call).await?;

    Ok(())
}
