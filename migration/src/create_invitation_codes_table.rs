use entity::invitation_codes::{Column, Entity};
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
                    .col(ColumnDef::new(Column::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Column::InvitationCode)
                            .char_len(6)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Column::Status).boolean().not_null())
                    .col(ColumnDef::new(Column::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(Column::UpdateAt).date_time().not_null())
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
