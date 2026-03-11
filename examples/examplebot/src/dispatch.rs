use fluxer_rs::{
    api::{FluxerApiHandler, channels::messages::SendMessageBuilder},
    error::FluxerRsError,
    gateway::dispatch::DispatchHandlerTrait,
    high_level::command_handler::CommandHandler,
    register_commands,
    serde::types::{
        guild::GuildCreateData,
        message::{
            EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder,
            EmbedMediaBuilder, MessageData,
        },
    },
};

use crate::commands::{
    addrole::AddRoleCommand, createrole::CreateRoleCommand, deleterole::DeleteRoleCommand,
    edit::EditCommand, ping::PingCommand, react::ReactCommand, remove_react::RemoveReactCommand,
    removerole::RemoveRoleCommand,
};

pub struct ColorbotDispatchHandler {}

impl DispatchHandlerTrait for ColorbotDispatchHandler {
    async fn handle_message_create_dispatch(
        &self,
        data: MessageData,
        api: &FluxerApiHandler,
    ) -> Result<(), FluxerRsError> {
        let mut cmd_handler = CommandHandler::init("!".to_string());

        register_commands!(cmd_handler,[
            {"addrole", AddRoleCommand},
            {"ping", PingCommand},
            {"edit", EditCommand},
            {"react", ReactCommand},
            {"removereact", RemoveReactCommand},
            {"removerole", RemoveRoleCommand},
            {"createrole", CreateRoleCommand},
            {"deleterole", DeleteRoleCommand},
        ]);

        cmd_handler.handle(&data, api).await
    }

    async fn handle_guild_create_dispatch(
        &self,
        data: GuildCreateData,
        api: &FluxerApiHandler,
    ) -> Result<(), FluxerRsError> {
        if data.id == "1477453554678525954" {
            api
                .execute_call(
                SendMessageBuilder::default()
                .content("Mhyello i am online".to_string())
                .channel_id("1477938338794254343".to_string())
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
                .build().unwrap()).await?;
        }
        Ok(())
    }
}
