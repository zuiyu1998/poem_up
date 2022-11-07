use poem::error::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]
pub enum Error {
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

pub type Result<T> = std::result::Result<T, Error>;
