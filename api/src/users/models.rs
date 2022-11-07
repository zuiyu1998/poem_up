use entity::users::ActiveModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserForm {
    #[validate(email, length(min = 10, max = 100))]
    pub email: String,
    #[validate(email, length(min = 8, max = 16))]
    pub password: String,
}

impl UserForm {
    pub fn into_active_model(&self) -> ActiveModel {
        todo!()
    }
}
