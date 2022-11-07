use super::models::UserForm;
use crate::Result;
use poem_up_service::Service;
use serde_json::{json, Value};
use validator::Validate;

pub async fn create(service: &Service, form: UserForm) -> Result<Value> {
    form.validate()?;

    let active = form.into_active_model();

    let user_service = service.user();

    let model = user_service.create(&active).await?;

    Ok(json!({
        "code": 200,
        "data": model,
    }))
}
