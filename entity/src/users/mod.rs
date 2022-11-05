use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Kind, Result};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nike_name: String,
    pub email: String,
    pub password: Vec<u8>,
    pub uid: String,
    pub is_delete: bool,
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
        } else if self.uid.is_set() {
            find_sql = find_sql.filter(Column::Uid.eq(self.uid.clone().into_value().unwrap()))
        } else if self.nike_name.is_set() {
            find_sql =
                find_sql.filter(Column::NikeName.eq(self.nike_name.clone().into_value().unwrap()))
        }

        if let Some(model) = find_sql.one(db).await? {
            return Ok(model);
        } else {
            return Err(Kind::UserNotFound);
        }
    }

    pub async fn update<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        self.find(conn).await?;
        let model = self.clone().update(conn).await?;
        Ok(model)
    }

    pub async fn create<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        match self.find(conn).await? {
            Ok(_) => {
                return Err(Kind::UserExist);
            }
            Err(e) => {
                if matches!(e, Kind::UserNotFound) {
                    self.insert(conn).await
                } else {
                    return Err(e);
                }
            }
        };
    }
}
