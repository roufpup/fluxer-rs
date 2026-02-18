use log::info;

use crate::gateway::dispatch_data::basic::SessionReplaceData;
use crate::gateway::dispatch_data::message::MessageReactData;
use crate::gateway::dispatch_data::ready::ReadyData;
use crate::gateway::dispatch_data::{
    guild::{GuildCreateData, GuildDeleteData},
    message::{MessageEventData, TypingEventData},
};

pub enum DispatchEvent {
    Ready(ReadyData),
    GuildDelete(GuildDeleteData),
    GuildCreate(Box<GuildCreateData>),
    MessageCreate(MessageEventData),
    MessageDelete(MessageEventData),
    MessageUpdate(MessageEventData),
    TypingStart(TypingEventData),
    TypingStop(TypingEventData),
    ReactionAdd(MessageReactData),
    ReactionRemove(MessageReactData),
    SessionReplace(Vec<SessionReplaceData>),
}

pub trait DispatchHandlerTrait {
    fn handle_ready_dispatch(&self, data: ReadyData) {
        info!("-> [DISPATCH::READY] BOT_NAME: {}", data.user.username)
    }

    fn handle_guild_delete_dispatch(&self, data: GuildDeleteData) {
        info!(
            "-> [DISPATCH::GUILD_DELETE] ID: {}, Unavailable: {}",
            data.guild_id, data.unavailable
        )
    }

    fn handle_guild_create_dispatch(&self, data: Box<GuildCreateData>) {
        info!("-> [DISPATCH::GUILD_CREATE] ID: {}", data.id)
    }

    fn handle_message_create_dispatch(&self, data: MessageEventData) {
        info!("-> [DISPATCH::MESSAGE_CREATE] CONTENT: {}", data.content)
    }

    fn handle_message_delete_dispatch(&self, data: MessageEventData) {
        info!("-> [DISPATCH::MESSAGE_DELETE] CONTENT: {}", data.content)
    }

    fn handle_message_update_dispatch(&self, data: MessageEventData) {
        info!("-> [DISPATCH::MESSAGE_UPDATE] CONTENT: {}", data.content)
    }

    fn handle_typing_start_dispatch(&self, data: TypingEventData) {
        info!(
            "-> [DISPATCH::TYPING_START] GUILD: {}, CHANNEL: {}",
            data.guild_id, data.channel_id
        )
    }

    fn handle_typing_stop_dispatch(&self, data: TypingEventData) {
        info!(
            "-> [DISPATCH::TYPING_STOP] GUILD: {}, CHANNEL: {}",
            data.guild_id, data.channel_id
        )
    }

    fn handle_message_reaction_add_dispatch(&self, data: MessageReactData) {
        info!(
            "-> [DISPATCH::MESSAGE_REACTION_ADD] MESSAGE: {}, EMOJI: {}",
            data.message_id, data.emoji.name
        )
    }

    fn handle_message_reaction_remove_dispatch(&self, data: MessageReactData) {
        info!(
            "-> [DISPATCH::MESSAGE_REACTION_REMOVE] MESSAGE: {}, EMOJI: {}",
            data.message_id, data.emoji.name
        )
    }

    fn handle_session_replace_dispatch(&self, _data: Vec<SessionReplaceData>) {
        info!("-> [DISPATCH::SESSION_REPLACE]",)
    }
}

#[derive(Default)]
pub struct DispatchHandler;
impl DispatchHandlerTrait for DispatchHandler {}

pub async fn handle_dispatch_events<T: DispatchHandlerTrait + Send + Sync + 'static>(
    dispatch_event: Box<DispatchEvent>,
    handler: &T,
) {
    match *dispatch_event {
        DispatchEvent::Ready(data) => handler.handle_ready_dispatch(data),
        DispatchEvent::GuildDelete(data) => handler.handle_guild_delete_dispatch(data),
        DispatchEvent::GuildCreate(data) => handler.handle_guild_create_dispatch(data),
        DispatchEvent::MessageCreate(data) => handler.handle_message_create_dispatch(data),
        DispatchEvent::MessageDelete(data) => handler.handle_message_delete_dispatch(data),
        DispatchEvent::MessageUpdate(data) => handler.handle_message_update_dispatch(data),
        DispatchEvent::TypingStart(data) => handler.handle_typing_start_dispatch(data),
        DispatchEvent::TypingStop(data) => handler.handle_typing_stop_dispatch(data),
        DispatchEvent::ReactionAdd(data) => handler.handle_message_reaction_add_dispatch(data),
        DispatchEvent::ReactionRemove(data) => {
            handler.handle_message_reaction_remove_dispatch(data)
        }
        DispatchEvent::SessionReplace(data) => handler.handle_session_replace_dispatch(data),
    }
}
