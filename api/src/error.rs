use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Kind(Kind),
    #[error("{0}")]
    FigmentError(figment::Error),
}

impl From<figment::Error> for Error {
    fn from(e: figment::Error) -> Self {
        Error::FigmentError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
