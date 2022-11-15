use crate::error::Result;
use entity::sea_orm::{ConnectionTrait, EntityTrait};
use entity::users::{ActiveModel, Model};

pub struct UserService<'a, C> {
    pub conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserService<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        UserService { conn }
    }

    pub async fn find(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.find(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn update(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.update(self.conn).await;

        res.map_err(|e| e.into())
    }

    pub async fn create(&self, active: &ActiveModel) -> Result<Model> {
        let res = active.create(self.conn).await;

        res.map_err(|e| e.into())
    }
}
