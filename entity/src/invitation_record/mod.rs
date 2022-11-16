use chrono::offset::Local;
use sea_orm::{entity::prelude::*, ConnectionTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Kind, Result};

#[derive(Clone, Deserialize, Serialize)]
pub struct InvitationRecord {
    pub user_id: i32,
    pub invitation_code: String,
}

impl InvitationRecord {
    pub fn new(user_id: i32, invitation_code: &str) -> Self {
        InvitationRecord {
            user_id,
            invitation_code: invitation_code.to_owned(),
        }
    }
}

impl IntoActiveModel<ActiveModel> for InvitationRecord {
    fn into_active_model(self) -> ActiveModel {
        let mut acitve: ActiveModel = Default::default();

        let now = Local::now();

        acitve.user_id = Set(self.user_id);
        acitve.invitation_code = Set(self.invitation_code);

        acitve.create_at = Set(now.naive_local());
        acitve.update_at = Set(now.naive_local());

        acitve
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "invitation_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub invitation_code: String,
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
            return Err(Kind::InvitationRecordNotFound.into());
        }
    }

    pub async fn create<C: ConnectionTrait>(&self, conn: &C) -> Result<Model> {
        let mut res_model = self.find(conn).await;

        res_model = match res_model {
            Ok(_) => Err(Kind::InvitationRecordExist.into()),

            Err(e) => {
                tracing::error!("error:{}", e);

                if matches!(e, Error::Kind(Kind::InvitationRecordNotFound)) {
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
