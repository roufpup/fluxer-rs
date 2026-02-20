use std::sync::Arc;

use fluxer_rs::{
    api::data_structure::{message::SendMessageBuilder, role::DeleteRoleBuilder},
    fluxerbot::FluxerBot,
    high_level::command_handler::{CommandHandler, CommandTrait},
};

pub struct DeleteRoleCommand {
    pub bot: Arc<FluxerBot>,
    pub channel_id: String,
    pub content: String,
}

impl CommandTrait for DeleteRoleCommand {
    async fn execute(&self) {
        if let Some((_, body)) = CommandHandler::remove_pfx("!", &self.content).await {
            let body_split = body.split(" ").collect::<Vec<&str>>();

            if body_split.len() != 2 || body.is_empty() {
                let _ = self.bot.api.execute_call(
                    SendMessageBuilder::default()
                        .channel_id(self.channel_id.clone())
                        .content("Invalid syntax")
                        .build()
                        .unwrap(),
                );
                return;
            }

            let _ = self.bot.api.execute_call(
                DeleteRoleBuilder::default()
                    .guild_id(body_split.first().unwrap().to_string())
                    .role_id(body_split.get(1).unwrap().to_string())
                    .build()
                    .unwrap(),
            );
        }
    }
}
