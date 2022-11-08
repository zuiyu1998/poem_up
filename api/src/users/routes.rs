use super::models::UserForm;
use poem::{
    handler, post,
    web::{Data, Form, Json},
    Endpoint, Result, Route,
};
use poem_up_service::Service;
use serde_json::Value;

use super::api;

//登录
#[handler]
pub async fn login(
    Data(service): Data<&Service>,
    Form(form): Form<UserForm>,
) -> Result<Json<Value>> {
    let res = api::login(service, form).await?;
    Ok(Json(res))
}
//创建用户
#[handler]
pub async fn create(
    Data(service): Data<&Service>,
    Form(form): Form<UserForm>,
) -> Result<Json<Value>> {
    let res = api::create(service, form).await?;
    Ok(Json(res))
}

pub fn new() -> impl Endpoint {
    Route::new()
        .at("/create", post(create))
        .at("/login", post(login))
}
