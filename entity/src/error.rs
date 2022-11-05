use thiserror::Error;

pub enum Kind {
    UserNotFound,
    UserExist,
}

#[derive(Debug, Error)]
pub enum Error {
    Kind(Kind),
}

pub type Result<T, Error> = std::result::Result<T, Error>;
