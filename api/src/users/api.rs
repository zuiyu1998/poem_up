use super::models::{UserForm, UserInfo, UserNikeNameUpdate};
use crate::{error::Kind, middlewares::service::encode, Result};
use entity::{invitation_codes, sea_orm::Set, users::Model as User};
use poem_up_service::Service;
use serde_json::{json, Value};
use validator::Validate;

pub async fn update_nike_name(
    service: &Service,
    user: &User,
    user_password_update: UserNikeNameUpdate,
) -> Result<Value> {
    user_password_update.validate()?;
    if user_password_update.nike_name == user.nike_name {
        return Err(Kind::NikeNameExists)?;
    }

    let transaction = service.transaction().await?;

    let mut active = user_password_update.into_active_model();

    let user_service = transaction.user();

    active.id = Set(user.id);

    user_service.update(&active).await?;
    transaction.commit().await?;

    Ok(json!({
        "code": 200,
    }))
}

pub async fn login(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let active = form.into_active_model();

    let transaction = service.transaction().await?;

    let user_service = transaction.user();

    let user = user_service.find(&active).await?;
    transaction.commit().await?;

    if !active.password.eq(&Set(user.password)) {
        return Err(Kind::PasswordError)?;
    }

    let token = encode(&user.uid)?;

    Ok(json!({
        "code": 200,
        "data": token,
    }))
}

pub async fn info(service: &Service, user: &User) -> Result<Value> {
    let transaction = service.transaction().await?;

    let invitation_code_service = transaction.invitation_code();

    let invitation_code = invitation_code_service.find_by_user_id(user.id).await?;
    transaction.commit().await?;

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
    let transaction = service.transaction().await?;

    let invitation_code_service = transaction.invitation_code();

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

    let user_service = transaction.user();

    user_service.create(&active).await?;
    transaction.commit().await?;

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
    let transaction = service.transaction().await?;

    let user_service = transaction.user();

    let user = user_service.create(&active).await?;

    let invitation_code_service = transaction.invitation_code();

    let user = invitation_code_service.create_by_user_id(user.id).await?;
    invitation_code_service
        .create_invitation_record(user.id, &form.code.unwrap())
        .await?;

    transaction.commit().await?;

    Ok(json!({
        "code": 200,
    }))
}
