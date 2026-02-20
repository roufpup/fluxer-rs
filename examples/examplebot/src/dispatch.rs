use std::sync::Arc;

use fluxer_rs::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, EmbedMediaBuilder,
    api::data_structure::message::SendMessageBuilder,
    fluxerbot::FluxerBot,
    gateway::{
        data_structure::{guild::GuildCreateData, message::MessageEventData},
        op_handlers::dispatch::DispatchHandlerTrait,
    },
    high_level::command_handler::CommandHandler,
};

use crate::commands::{
    addrole::AddRoleCommand, createrole::CreateRoleCommand, deleterole::DeleteRoleCommand, edit::EditCommand, ping::PingCommand, react::ReactCommand, remove_react::RemoveReactCommand, removerole::RemoveRoleCommand
};

pub struct ColorbotDispatchHandler {
    pub bot: Arc<FluxerBot>,
}

impl DispatchHandlerTrait for ColorbotDispatchHandler {
    async fn handle_message_create_dispatch(&self, data: MessageEventData) {
        let mut cmd_handler = CommandHandler::init("!".to_string());

        cmd_handler.register_command(
            "ping".to_string(),
            PingCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                id: data.id.clone(),
            },
        );
        cmd_handler.register_command(
            "edit".to_string(),
            EditCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "react".to_string(),
            ReactCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "removereact".to_string(),
            RemoveReactCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "addrole".to_string(),
            AddRoleCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "removerole".to_string(),
            RemoveRoleCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "createrole".to_string(),
            CreateRoleCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );
        cmd_handler.register_command(
            "deleterole".to_string(),
            DeleteRoleCommand {
                bot: self.bot.clone(),
                channel_id: data.channel_id.clone(),
                content: data.content.clone(),
            },
        );

        cmd_handler.handle(&data).await;
    }

    async fn handle_guild_create_dispatch(&self, data: Box<GuildCreateData>) {
        if data.id == "1473686979970875591" {
            let _ = self.bot
                .api
                .execute_call(
                SendMessageBuilder::default()
                .content("Mhyello i am online".to_string())
                .channel_id("1474424011696861241".to_string())
                .embeds(vec![
                    EmbedBuilder::default()
                        .title("Added to Queue".to_string())
                        .color(0x4d6fb7)
                        .fields(vec![
                        EmbedFieldBuilder::default().name("Tracka".to_string()).value("Sussy bakka").inline(true).build().unwrap(),
                        EmbedFieldBuilder::default().name("Artist".to_string()).value("Sussy bakka").inline(true).build().unwrap(),
                        EmbedFieldBuilder::default().name("Durationa".to_string()).value("Sussy bakka").inline(true).build().unwrap(),
                        EmbedFieldBuilder::default().name("Position in Queue".to_string()).value("Sussy bakka").inline(true).build().unwrap()
                        ])
                        .footer(EmbedFooterBuilder::default().text("Requested by the goober".to_string()).build().unwrap())
                        .author(EmbedAuthorBuilder::default().name("GOOB".to_string()).icon_url("https://fluxerusercontent.com/avatars/1472242534880579616/4c64d1e4.webp".to_string()).build().unwrap())
                        .image(EmbedMediaBuilder::default().url("https://s3.animalia.bio/animals/photos/full/1.25x1/sahra11jpg.webp".to_string()).build().unwrap())
                        .build()
                        .unwrap(),
                    ]
                )
                .message_reference(None)
                .build().unwrap());
        }
    }
}
