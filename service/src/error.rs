use thiserror::Error;

pub enum Kind {
    UserNotFound,
    UserExist,
}

#[derive(Debug, Error)]
pub enum Error {
    Kind(Kind),
    EntityError(entity::error::Error),
}

impl From<entity::error::Error> for Error {
    fn from(e: entity::error::Error) -> Self {
        Error::EntityError(e)
    }
}

pub type Result<T, Error> = std::result::Result<T, Error>;
