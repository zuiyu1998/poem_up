use crate::error::{Error, Result};
use entity::users::{ActiveModel, Entity, Model};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, TransactionTrait};

#[derive(Debug, Clone, Copy)]
pub struct UserListOption {
    page: i32,
    page_size: i32,
}

pub struct UserList {
    pub data: Vec<Model>,
    pub total: i32,
    pub option: UserListOption,
}

pub struct UserService {
    pub conn: &mut DatabaseConnection,
}

impl UserService {
    pub fn new(&self, conn: &mut DatabaseConnection) -> Self {
        UserService { conn }
    }

    pub fn find(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.find(&conn).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub fn update(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.update(&conn).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub fn create(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.create(&conn).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub fn list(&self, list_option: &UserListOption) -> Result<UserList> {
        let begin = self.conn.begin().await?;

        let paginate = Entity::find().paginate(&begin, list_option.page_size);

        let total = paginate.num_items().await?;

        let data = paginate.fetch_page(list_option.page).await?;

        begin.commit().await?;

        Ok(UserList {
            data,
            total,
            option,
        })
    }
}
