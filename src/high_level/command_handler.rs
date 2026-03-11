use std::{collections::HashMap, pin::Pin};

use anyhow::Result;

use crate::{
    api::FluxerApiHandler,
    error::{CommandHandlerError, FluxerRsError},
    serde::types::message::MessageData,
};

pub type Command = Box<dyn BoxedCommandTrait + Send + Sync>;

pub trait CommandTrait: Send + Sync + 'static {
    fn execute<'a>(
        &'a self,
        api: &'a FluxerApiHandler,
        feedback: &'a CommandFeedback<'a>,
    ) -> impl Future<Output = Result<(), FluxerRsError>> + Send + 'a;
}

pub trait BoxedCommandTrait {
    fn execute<'a>(
        &'a self,
        api: &'a FluxerApiHandler,
        feedback: &'a CommandFeedback<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), FluxerRsError>> + Send + 'a>>;
}

impl<C: CommandTrait> BoxedCommandTrait for C {
    fn execute<'a>(
        &'a self,
        api: &'a FluxerApiHandler,
        feedback: &'a CommandFeedback<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), FluxerRsError>> + Send + 'a>> {
        Box::pin(CommandTrait::execute(self, api, feedback))
    }
}

pub struct CommandHandler {
    prefix: String,
    map: HashMap<String, Command>,
}

pub struct CommandFeedback<'a> {
    pub data: &'a MessageData,
    pub args: Vec<&'a str>,
}

impl CommandHandler {
    pub fn init(prefix: impl Into<String>) -> Self {
        CommandHandler {
            prefix: prefix.into(),
            map: HashMap::new(),
        }
    }

    pub fn register_command(
        &mut self,
        command_name: impl Into<String>,
        handler: impl CommandTrait + 'static,
    ) {
        self.map.insert(command_name.into(), Box::new(handler));
    }

    pub async fn handle(
        &self,
        data: &MessageData,
        api: &FluxerApiHandler,
    ) -> Result<(), FluxerRsError> {
        // Checking whether the message content really starts with the prefix and if it's not just the prefix itself
        if !data.content.starts_with(&self.prefix) || data.content.len() < 2 {
            return Ok(());
        }

        let (cmd, args) = Self::command_data(&self.prefix, &data.content)?;

        let split_args = Self::split_args(args);

        match self.map.get(cmd) {
            Some(handler) => {
                handler
                    .execute(
                        api,
                        &CommandFeedback {
                            data,
                            args: split_args,
                        },
                    )
                    .await
            }
            None => Err(FluxerRsError::CommandHandlerError(
                crate::error::CommandHandlerError::UnknownCommand(args.to_string()),
            )),
        }
    }

    pub fn command_data<'a>(
        prefix: &str,
        content: &'a str,
    ) -> Result<(&'a str, &'a str), FluxerRsError> {
        content
            .strip_prefix(prefix)
            .ok_or_else(|| CommandHandlerError::Custom("Failed to remove command prefix".into()))
            .map(|stripped_body| match stripped_body.split_once(" ") {
                Some((cmd, args)) => (cmd, args),
                None => (stripped_body, ""),
            })
            .map_err(FluxerRsError::from)
    }

    pub fn split_args(body: &str) -> Vec<&str> {
        body.split(" ").collect::<Vec<&str>>()
    }
}
