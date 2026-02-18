use log::info;

use crate::gateway::dispatch_data::{
    guild::{GuildCreateData, GuildDeleteData},
    message::{MessageEventData, TypingEventData},
};

pub enum DispatchEvent {
    Ready,
    GuildDelete(GuildDeleteData),
    GuildCreate(Box<GuildCreateData>),
    MessageCreate(MessageEventData),
    MessageDelete(MessageEventData),
    TypingStart(TypingEventData),
    TypingStop(TypingEventData),
}

pub trait DispatchHandlerTrait {
    fn handle_ready_dispatch(&self) {
        info!("-> [DISPATCH::READY]")
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
}

#[derive(Default)]
pub struct DispatchHandler;
impl DispatchHandlerTrait for DispatchHandler {}

pub async fn handle_dispatch_events<T: DispatchHandlerTrait + Send + Sync + 'static>(
    dispatch_event: Box<DispatchEvent>,
    dispatcher: &T,
) {
    match *dispatch_event {
        DispatchEvent::Ready => dispatcher.handle_ready_dispatch(),
        DispatchEvent::GuildDelete(data) => dispatcher.handle_guild_delete_dispatch(data),
        DispatchEvent::GuildCreate(data) => dispatcher.handle_guild_create_dispatch(data),
        DispatchEvent::MessageCreate(data) => dispatcher.handle_message_create_dispatch(data),
        DispatchEvent::MessageDelete(data) => dispatcher.handle_message_delete_dispatch(data),
        DispatchEvent::TypingStart(data) => dispatcher.handle_typing_start_dispatch(data),
        DispatchEvent::TypingStop(data) => dispatcher.handle_typing_stop_dispatch(data),
    }
}
