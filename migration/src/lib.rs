pub use sea_orm_migration::prelude::*;

mod create_invitation_codes_table;
mod create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_user_table::Migration),
            Box::new(create_invitation_codes_table::Migration),
        ]
    }
}
