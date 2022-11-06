use entity::users::{Column, Entity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::NikeName).char_len(20).not_null())
                    .col(ColumnDef::new(Column::Email).char_len(100).not_null())
                    .col(ColumnDef::new(Column::Password).binary().not_null())
                    .col(ColumnDef::new(Column::Uid).char_len(30).not_null())
                    .col(ColumnDef::new(Column::IsDelete).boolean().not_null())
                    .col(ColumnDef::new(Column::CreateAt).timestamp().not_null())
                    .col(ColumnDef::new(Column::UpdateAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
