use fluxer_rs_macros::dispatch;

dispatch!(
    ["READY", ReadyData],
    ["GUILD_DELETE", GuildDeleteData],
    ["GUILD_CREATE", GuildCreateData],
    ["GUILD_UPDATE", GuildPropertiesData],
    ["GUILD_EMOJIS_UPDATE", GuildEmojisUpdateData],
    ["GUILD_ROLE_CREATE", GuildRoleCreateData],
    ["GUILD_ROLE_UPDATE", GuildRoleCreateData],
    ["GUILD_ROLE_UPDATE_BULK", GuildRoleUpdateBulkData],
    ["GUILD_ROLE_DELETE", GuildRoleDeleteData],
    ["GUILD_MEMBER_UPDATE", MemberData],
    ["MESSAGE_CREATE", MessageData],
    ["MESSAGE_DELETE", MessageData],
    ["MESSAGE_UPDATE", MessageData],
    ["TYPING_START", TypingStartData],
    ["MESSAGE_REACTION_ADD", MessageReactionData],
    ["MESSAGE_REACTION_REMOVE", MessageReactionData],
    ["MESSAGE_REACTION_REMOVE_EMOJI", MessageReactionData],
    ["SESSIONS_REPLACE", Vec<SessionReplaceData>],
    ["CHANNEL_UPDATE", ChannelData],
    ["None", Option],
);
