use sea_orm::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("UserNotFound")]
    UserNotFound,
    #[error("UserExist")]
    UserExist,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Kind(Kind),
    #[error("{0}")]
    EntityError(entity::error::Error),
}

impl From<entity::error::Error> for Error {
    fn from(e: entity::error::Error) -> Self {
        Error::EntityError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
