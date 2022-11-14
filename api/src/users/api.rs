use super::models::{UserForm, UserInfo};
use crate::{error::Kind, middlewares::service::encode, Result};
use entity::{invitation_codes, sea_orm::Set};
use poem_up_service::Service;
use serde_json::{json, Value};
use validator::Validate;

pub async fn login(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let active = form.into_active_model();

    let user_service = service.user();

    let user = user_service.find(&active).await?;

    let token = encode(&user.uid)?;

    Ok(json!({
        "code": 200,
        "data": token,
    }))
}

pub async fn info(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let active = form.into_active_model();

    let user_service = service.user();

    let user = user_service.find(&active).await?;

    let invitation_code_service = service.invitation_code();

    let invitation_code = invitation_code_service.find_by_user_id(user.id).await?;

    let user_info = UserInfo::new(&user, &invitation_code);

    Ok(json!({
        "code": 200,
        "data": user_info,
    }))
}

pub async fn create_by_code(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    if form.code.is_none() {
        return Err(Kind::CodeNotFound.into());
    }

    let invitation_code_service = service.invitation_code();

    let mut active = invitation_codes::ActiveModel::default();
    active.invitation_code = Set(form.code.clone().unwrap());

    if !invitation_code_service.is_valid(&active).await? {
        return Err(Kind::CodeNotValid.into());
    }

    let mut active = form.into_active_model();

    let uid = super::uid();

    let nike_name = String::from("uid_") + &uid;

    active.uid = Set(uid);
    active.nike_name = Set(nike_name);

    let user_service = service.user();

    user_service.create(&active).await?;

    Ok(json!({
        "code": 200,
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

    let user = user_service.create(&active).await?;

    let invitation_code_service = service.invitation_code();

    invitation_code_service.create_by_user_id(user.id).await?;

    Ok(json!({
        "code": 200,
    }))
}
