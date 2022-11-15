use crate::error::{Error, Result};
use entity::sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use invitation_codes::InvitationCodeService;
use user::UserService;

pub mod error;
pub mod invitation_codes;
pub mod user;

#[derive(Debug, Clone)]
pub struct Service {
    conn: DatabaseConnection,
}

pub struct Transaction(DatabaseTransaction);

impl Transaction {
    pub async fn begin(conn: &DatabaseConnection) -> Result<Self> {
        let begin = conn.begin().await?;
        Ok(Transaction(begin))
    }

    pub fn user(&self) -> UserService<DatabaseTransaction> {
        UserService::new(&self.0)
    }

    pub fn invitation_code(&self) -> InvitationCodeService<DatabaseTransaction> {
        InvitationCodeService::new(&self.0)
    }

    pub async fn commit(self) -> Result<()> {
        self.0.commit().await?;
        Ok(())
    }
}

impl Service {
    pub fn new(conn: DatabaseConnection) -> Self {
        Service { conn }
    }

    pub async fn transaction(&self) -> Result<Transaction> {
        Transaction::begin(&self.conn).await
    }
}
