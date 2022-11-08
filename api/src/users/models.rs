use chrono::offset::Local;
use entity::sea_orm::entity::Set;
use entity::users::ActiveModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserForm {
    #[validate(email, length(min = 10, max = 100))]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
}

impl UserForm {
    pub fn into_active_model(&self) -> ActiveModel {
        let mut active = ActiveModel::default();

        let now = Local::now();

        let password = super::spawn_password(&self.password);

        active.password = Set(password);
        active.email = Set(self.email.clone());

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        active
    }
}
