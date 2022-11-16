use thiserror::Error;

#[derive(Debug, Error)]
pub enum Kind {
    #[error("InvitationCodeNotFound")]
    InvitationCodeNotFound,
    #[error("InvitationCodeExist")]
    InvitationCodeExist,
    #[error("UserNotFound")]
    UserNotFound,
    #[error("UserExist")]
    UserExist,
    #[error("DynamicNotFound")]
    DynamicNotFound,
    #[error("InvitationRecordNotFound")]
    InvitationRecordNotFound,
    #[error("InvitationRecordExist")]
    InvitationRecordExist,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Kind(Kind),
    #[error("{0}")]
    DbErr(sea_orm::DbErr),
}

impl From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Error::DbErr(e)
    }
}

impl From<Kind> for Error {
    fn from(e: Kind) -> Self {
        Error::Kind(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
