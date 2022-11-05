use sea_orm::DatabaseConnection;

pub mod error;
pub mod user;

pub struct Service {
    conn: DatabaseConnection,
}
