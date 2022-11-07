use crate::error::Result;
use entity::sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, TransactionTrait};
use entity::users::{ActiveModel, Entity, Model};

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

pub struct UserService<'a> {
    pub conn: &'a DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        UserService { conn }
    }

    pub async fn find(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.find(&begin).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub async fn update(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.update(&begin).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub async fn create(&self, active: &ActiveModel) -> Result<Model> {
        let begin = self.conn.begin().await?;

        let res = active.create(&begin).await;

        begin.commit().await?;

        res.map_err(|e| e.into())
    }

    pub async fn list(&self, list_option: &UserListOption) -> Result<UserList> {
        let begin = self.conn.begin().await?;

        let paginate = Entity::find().paginate(&begin, list_option.page_size as usize);

        let total = paginate.num_items().await?;

        let data = paginate.fetch_page(list_option.page as usize).await?;

        begin.commit().await?;

        Ok(UserList {
            data,
            total: total as i32,
            option: list_option.to_owned(),
        })
    }
}
