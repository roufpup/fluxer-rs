use std::sync::Arc;

use fluxer_rs::{
    api::data_structure::message::{MessageReferenceBuilder, SendMessageBuilder},
    fluxerbot::FluxerBot,
    high_level::command_handler::CommandTrait,
};

pub struct PingCommand {
    pub bot: Arc<FluxerBot>,
    pub channel_id: String,
    pub id: String,
}

impl CommandTrait for PingCommand {
    async fn execute(&self) {
        let _ = self.bot.api.execute_call(
            SendMessageBuilder::default()
                .channel_id(self.channel_id.clone())
                .content("pong".to_string())
                .message_reference(
                    MessageReferenceBuilder::default()
                        .message_id(self.id.clone())
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );
    }
}
