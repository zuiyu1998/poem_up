use poem::error::ResponseError;
use thiserror::Error;

use crate::middlewares::{AuthError, ServiceDbError};

#[derive(Debug, Error)]
pub enum Kind {
    #[error("CodeNotFound")]
    CodeNotFound,
    #[error("CodeNotValid")]
    CodeNotValid,
    #[error("PasswordError")]
    PasswordError,
    #[error("NikeNameExists")]
    NikeNameExists,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ServiceDbError(ServiceDbError),
    #[error("{0}")]
    AuthError(AuthError),
    #[error("{0}")]
    Kind(Kind),
    #[error("{0}")]
    FigmentError(figment::Error),
    #[error("{0}")]
    ValidateError(validator::ValidationErrors),
    #[error("{0}")]
    ServiceError(poem_up_service::error::Error),
}

impl ResponseError for Error {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::OK
    }
}

impl From<poem_up_service::error::Error> for Error {
    fn from(e: poem_up_service::error::Error) -> Self {
        Error::ServiceError(e)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error::ValidateError(e)
    }
}

impl From<figment::Error> for Error {
    fn from(e: figment::Error) -> Self {
        Error::FigmentError(e)
    }
}

impl From<Kind> for Error {
    fn from(e: Kind) -> Self {
        Error::Kind(e)
    }
}

impl From<ServiceDbError> for Error {
    fn from(e: ServiceDbError) -> Self {
        Error::ServiceDbError(e)
    }
}

impl From<AuthError> for Error {
    fn from(e: AuthError) -> Self {
        Error::AuthError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
