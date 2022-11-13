use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Kind, Result};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "invitation_codes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub invitation_code: String,
    pub status: bool,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub async fn find<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        let mut find_sql = Entity::find();
        if self.id.is_set() {
            find_sql = find_sql.filter(Column::Id.eq(self.id.clone().into_value().unwrap()))
        } else if self.user_id.is_set() {
            find_sql =
                find_sql.filter(Column::UserId.eq(self.user_id.clone().into_value().unwrap()))
        } else if self.invitation_code.is_set() {
            find_sql = find_sql.filter(
                Column::InvitationCode.eq(self.invitation_code.clone().into_value().unwrap()),
            )
        }

        if let Some(model) = find_sql.one(conn).await? {
            return Ok(model);
        } else {
            return Err(Kind::InvitationCodeNotFound.into());
        }
    }

    pub async fn update<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        self.find(conn).await?;
        let model = self.clone().update(conn).await?;
        Ok(model)
    }

    pub async fn create<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        let mut res_model = self.find(conn).await;

        res_model = match res_model {
            Ok(_) => Err(Kind::InvitationCodeExist.into()),

            Err(e) => {
                tracing::error!("error:{}", e);

                if matches!(e, Error::Kind(Kind::InvitationCodeNotFound)) {
                    let model = self.clone().insert(conn).await?;

                    Ok(model)
                } else {
                    Err(e)
                }
            }
        };

        res_model
    }
}
