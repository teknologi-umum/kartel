use std::{fmt::Debug, sync::Arc};

use teloxide::{ApiError, RequestError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Telegram Error: {0}")]
    TelegramError(#[from] RequestError),

    #[error("Invalid Arguments: {0}")]
    InvalidArguments(anyhow::Error),

    #[error("API call error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("API call return error response: {0}")]
    ApiError(anyhow::Error),
}

pub trait AsClientError<T> {
    fn as_client_err(self) -> Result<T, HandlerError>;
}

pub trait AsInternalError<T> {
    fn as_internal_err(self) -> Result<T, HandlerError>;
}

impl<T, E> AsClientError<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn as_client_err(self) -> Result<T, HandlerError> {
        self.map_err(|e| HandlerError::InvalidArguments(e.into()))
    }
}

impl<T, E> AsInternalError<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn as_internal_err(self) -> Result<T, HandlerError> {
        self.map_err(|e| HandlerError::ApiError(e.into()))
    }
}

impl From<HandlerError> for RequestError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::TelegramError(err) => err,
            HandlerError::NetworkError(err) => RequestError::Network(Arc::new(err)),
            HandlerError::InvalidArguments(err) => {
                RequestError::Api(ApiError::Unknown(err.to_string()))
            }
            HandlerError::ApiError(err) => RequestError::Api(ApiError::Unknown(err.to_string())),
        }
    }
}
