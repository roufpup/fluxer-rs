use async_trait::async_trait;
use log::info;

use crate::gateway::data_structure::common::{ChannelData, GuildPropertiesData, MemberData};
use crate::gateway::data_structure::guild::{
    GuildEmojisUpdateData, GuildRoleCreateData, GuildRoleDeleteData, GuildRoleUpdateBulkData,
};
use crate::gateway::data_structure::message::MessageReactData;
use crate::gateway::data_structure::user::ReadyData;
use crate::gateway::data_structure::user::SessionReplaceData;
use crate::gateway::data_structure::{
    guild::{GuildCreateData, GuildDeleteData},
    message::{MessageEventData, TypingEventData},
};

pub enum DispatchEvent {
    Ready(Box<ReadyData>),
    GuildDelete(GuildDeleteData),
    GuildCreate(Box<GuildCreateData>),
    GuildUpdate(GuildPropertiesData),
    GuildEmojisUpdate(GuildEmojisUpdateData),
    GuildRoleCreate(GuildRoleCreateData),
    GuildRoleUpdate(GuildRoleCreateData),
    GuildRoleUpdateBulk(GuildRoleUpdateBulkData),
    GuildRoleDelete(GuildRoleDeleteData),
    GuildMemberUpdate(MemberData),
    MessageCreate(MessageEventData),
    MessageDelete(MessageEventData),
    MessageUpdate(MessageEventData),
    TypingStart(TypingEventData),
    TypingStop(TypingEventData),
    MessageReactionAdd(MessageReactData),
    MessageReactionRemove(MessageReactData),
    MessageReactionRemoveEmoji(MessageReactData),
    SessionReplace(Vec<SessionReplaceData>),
    ChannelUpdate(ChannelData),
}

pub trait DispatchHandlerTrait {
    fn handle_ready_dispatch(&self, _data: Box<ReadyData>) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::READY]") }
    }

    fn handle_guild_delete_dispatch(
        &self,
        _data: GuildDeleteData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_DELETE]") }
    }

    fn handle_guild_create_dispatch(
        &self,
        _data: Box<GuildCreateData>,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_CREATE]") }
    }

    fn handle_guild_update_dispatch(
        &self,
        _data: GuildPropertiesData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_UPDATE]") }
    }

    fn handle_guild_emojis_update_dispatch(
        &self,
        _data: GuildEmojisUpdateData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_EMOJIS_UPDATE]") }
    }

    fn handle_guild_role_create_dispatch(
        &self,
        _data: GuildRoleCreateData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_ROLE_CREATE]") }
    }

    fn handle_guild_role_update_dispatch(
        &self,
        _data: GuildRoleCreateData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_ROLE_UPDATE]") }
    }

    fn handle_guild_role_update_bulk_dispatch(
        &self,
        _data: GuildRoleUpdateBulkData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_ROLE_UPDATE_BULK]") }
    }

    fn handle_guild_role_delete_dispatch(
        &self,
        _data: GuildRoleDeleteData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_ROLE_DELETE]") }
    }

    fn handle_guild_member_update_dispatch(
        &self,
        _data: MemberData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::GUILD_MEMBER_UPDATE]") }
    }

    fn handle_message_create_dispatch(
        &self,
        _data: MessageEventData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_CREATE]") }
    }

    fn handle_message_delete_dispatch(
        &self,
        _data: MessageEventData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_DELETE]") }
    }

    fn handle_message_update_dispatch(
        &self,
        _data: MessageEventData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_UPDATE]") }
    }

    fn handle_typing_start_dispatch(
        &self,
        _data: TypingEventData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::TYPING_START]") }
    }

    fn handle_typing_stop_dispatch(
        &self,
        _data: TypingEventData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::TYPING_STOP]") }
    }

    fn handle_message_reaction_add_dispatch(
        &self,
        _data: MessageReactData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_REACTION_ADD]") }
    }

    fn handle_message_reaction_remove_dispatch(
        &self,
        _data: MessageReactData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_REACTION_REMOVE]") }
    }
    fn handle_message_reaction_remove_emoji_dispatch(
        &self,
        _data: MessageReactData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::MESSAGE_REACTION_REMOVE_EMOJI]") }
    }

    fn handle_session_replace_dispatch(
        &self,
        _data: Vec<SessionReplaceData>,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::SESSION_REPLACE]",) }
    }

    fn handle_channel_update_dispatch(
        &self,
        _data: ChannelData,
    ) -> impl Future<Output = ()> + Send {
        async move { info!("-> [DISPATCH::CHANNEL_UPDATE]",) }
    }
}

#[derive(Default)]
pub struct DispatchHandler;
#[async_trait]
impl DispatchHandlerTrait for DispatchHandler {}

pub async fn handle_dispatch_events<T: DispatchHandlerTrait + Send + Sync + 'static>(
    dispatch_event: Box<DispatchEvent>,
    handler: &T,
) {
    match *dispatch_event {
        DispatchEvent::Ready(data) => handler.handle_ready_dispatch(data).await,
        DispatchEvent::GuildDelete(data) => handler.handle_guild_delete_dispatch(data).await,
        DispatchEvent::GuildCreate(data) => handler.handle_guild_create_dispatch(data).await,
        DispatchEvent::GuildUpdate(data) => handler.handle_guild_update_dispatch(data).await,
        DispatchEvent::GuildEmojisUpdate(data) => {
            handler.handle_guild_emojis_update_dispatch(data).await
        }
        DispatchEvent::GuildRoleCreate(data) => {
            handler.handle_guild_role_create_dispatch(data).await
        }
        DispatchEvent::GuildRoleUpdate(data) => {
            handler.handle_guild_role_update_dispatch(data).await
        }
        DispatchEvent::GuildRoleUpdateBulk(data) => {
            handler.handle_guild_role_update_bulk_dispatch(data).await
        }
        DispatchEvent::GuildRoleDelete(data) => {
            handler.handle_guild_role_delete_dispatch(data).await
        }
        DispatchEvent::GuildMemberUpdate(data) => {
            handler.handle_guild_member_update_dispatch(data).await
        }
        DispatchEvent::MessageCreate(data) => handler.handle_message_create_dispatch(data).await,
        DispatchEvent::MessageDelete(data) => handler.handle_message_delete_dispatch(data).await,
        DispatchEvent::MessageUpdate(data) => handler.handle_message_update_dispatch(data).await,
        DispatchEvent::MessageReactionRemoveEmoji(data) => {
            handler
                .handle_message_reaction_remove_emoji_dispatch(data)
                .await
        }
        DispatchEvent::TypingStart(data) => handler.handle_typing_start_dispatch(data).await,
        DispatchEvent::TypingStop(data) => handler.handle_typing_stop_dispatch(data).await,
        DispatchEvent::MessageReactionAdd(data) => {
            handler.handle_message_reaction_add_dispatch(data).await
        }
        DispatchEvent::MessageReactionRemove(data) => {
            handler.handle_message_reaction_remove_dispatch(data).await
        }
        DispatchEvent::SessionReplace(data) => handler.handle_session_replace_dispatch(data).await,
        DispatchEvent::ChannelUpdate(data) => handler.handle_channel_update_dispatch(data).await,
    }
}
