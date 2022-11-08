use super::models::UserForm;
use crate::Result;
use entity::sea_orm::Set;
use poem_up_service::Service;
use serde_json::{json, Value};
use validator::Validate;

pub async fn login(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let active = form.into_active_model();

    let user_service = service.user();

    let model = user_service.find(&active).await?;

    Ok(json!({
        "code": 200,
        "data": model,
    }))
}

pub async fn create(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let mut active = form.into_active_model();
    let uid = super::uid();

    let nike_name = String::from("uid_") + &uid;

    active.uid = Set(uid);
    active.nike_name = Set(nike_name);

    active.is_delete = Set(false);

    let user_service = service.user();

    let model = user_service.create(&active).await?;

    Ok(json!({
        "code": 200,
        "data": model,
    }))
}
