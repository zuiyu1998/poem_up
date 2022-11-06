use entity::sea_orm;
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
    #[error("{0}")]
    DbErr(sea_orm::DbErr),
}

impl From<entity::error::Error> for Error {
    fn from(e: entity::error::Error) -> Self {
        Error::EntityError(e)
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Error::DbErr(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
