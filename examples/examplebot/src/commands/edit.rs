use std::sync::Arc;

use fluxer_rs::{api::data_structure::message::{EditMessageBuilder, FetchMessage, FetchMessageBuilder, SendMessageBuilder}, fluxerbot::FluxerBot, gateway::data_structure::message::MessageEventData, high_level::command_handler::{CommandHandler, CommandTrait}};

pub struct EditCommand {
    pub bot: Arc<FluxerBot>,
    pub channel_id: String,
    pub content: String,
}

impl CommandTrait for EditCommand {
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

            let resp = self.bot.api.execute_call(
                FetchMessageBuilder::default()
                    .channel_id(body_split.first().unwrap().to_string())
                    .message_id(body_split.get(1).unwrap().to_string())
                    .build()
                    .unwrap(),
            );

            let message_info: Option<MessageEventData> = match resp {
                Ok(value) => Some(FetchMessage::get_resp(value.as_str().unwrap())),
                Err(err) => {
                    let _ = self.bot.api.execute_call(
                        SendMessageBuilder::default()
                            .channel_id(self.channel_id.clone())
                            .content(err.to_string())
                            .build()
                            .unwrap(),
                    );
                    None
                }
            };

            if let Some(value) = message_info {
                let _ = self.bot.api.execute_call(
                    EditMessageBuilder::default()
                        .message_id(value.id)
                        .channel_id(value.channel_id)
                        .embeds(value.embeds)
                        .content("EEEEEEEEEEEEE")
                        .build()
                        .unwrap(),
                );
            }
        }
    }
}