use entity::sea_orm::DatabaseConnection;

pub mod error;
pub mod user;

#[derive(Debug, Clone)]
pub struct Service {
    conn: DatabaseConnection,
}

impl Service {
    pub fn new(conn: DatabaseConnection) -> Self {
        Service { conn }
    }
}
