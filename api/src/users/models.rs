use chrono::offset::Local;
use entity::sea_orm::entity::{prelude::*, Set};
use entity::{
    invitation_codes::Model as InvitationCode,
    users::{ActiveModel, Model as User},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserForm {
    #[validate(email, length(min = 10, max = 100))]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
    #[validate(length(min = 6, max = 6))]
    pub code: Option<String>,
}

impl UserForm {
    pub fn into_active_model(&self) -> ActiveModel {
        let mut active: ActiveModel = Default::default();

        let now = Local::now();

        let password = super::spawn_password(&self.password);

        active.is_delete = Set(false);

        active.password = Set(password);
        active.email = Set(self.email.clone());

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        active
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub nike_name: String,
    pub email: String,
    pub uid: String,
    pub is_delete: bool,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub invitation_code: InvitationCode,
}

impl UserInfo {
    pub fn new(user: &User, invitation_code: &InvitationCode) -> Self {
        UserInfo {
            id: user.id,
            nike_name: user.nike_name.to_owned(),
            email: user.email.to_owned(),
            uid: user.uid.to_owned(),
            is_delete: user.is_delete,
            create_at: user.create_at,
            update_at: user.update_at,
            invitation_code: invitation_code.clone(),
        }
    }
}
