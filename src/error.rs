use std::num::ParseIntError;

use crate::{
    api::{
        FluxerApiHandlerBuilderError,
        channels::{
            messages::{
                EditMessageBuilderError, FetchMessageBuilderError, SendMessageBuilderError,
            },
            reactions::{AddOwnReactionBuilderError, RemoveAllEmojiReactionsBuilderError},
        },
        guilds::roles::{
            AddRoleToMemberBuilderError, CreateRoleBuilderError, DeleteRoleBuilderError,
            RemoveRoleFromMemberBuilderError,
        },
    },
    serde::types::message::MessageReferenceBuilderError,
};

#[derive(Debug, thiserror::Error)]
pub enum FluxerRsError {
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    SendError(String),
    // Custom
    #[error("{0}")]
    ApiHandlerError(#[from] ApiHandlerError),
    #[error("{0}")]
    CommandHandlerError(#[from] CommandHandlerError),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandHandlerError {
    #[error("The command {0} is unknown.")]
    UnknownCommand(String),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiHandlerError {
    #[error("{0}")]
    FluxerApiHandlerBuilderError(#[from] FluxerApiHandlerBuilderError),
    #[error("{0}")]
    MessageReferenceBuilderError(#[from] MessageReferenceBuilderError),
    #[error("{0}")]
    FetchMessageBuilderError(#[from] FetchMessageBuilderError),
    #[error("{0}")]
    SendMessageBuilderError(#[from] SendMessageBuilderError),
    #[error("{0}")]
    EditMessageBuilderError(#[from] EditMessageBuilderError),
    #[error("{0}")]
    AddRoleToMemberBuilderError(#[from] AddRoleToMemberBuilderError),
    #[error("{0}")]
    RemoveRoleFromMemberBuilderError(#[from] RemoveRoleFromMemberBuilderError),
    #[error("{0}")]
    CreateRoleBuilderError(#[from] CreateRoleBuilderError),
    #[error("{0}")]
    DeleteRoleBuilderError(#[from] DeleteRoleBuilderError),
    #[error("{0}")]
    AddOwnReactionBuilderError(#[from] AddOwnReactionBuilderError),
    #[error("{0}")]
    RemoveAllEmojiReactionsBuilderError(#[from] RemoveAllEmojiReactionsBuilderError),
}
