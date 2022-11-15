use crate::middlewares::{service::ServiceDb, Auth, Bearer};

use super::models::{UserForm, UserNikeNameUpdate};
use entity::users::Model as User;
use poem::{
    get, handler, post,
    web::{Data, Form, Json},
    Endpoint, EndpointExt, Result, Route,
};

use poem_up_service::Service;
use serde_json::Value;

use super::api;

//更改昵称
#[handler]
pub async fn update_nike_name(
    Data(service): Data<&Service>,
    Data(user): Data<&User>,
    Form(user_nike_name_update): Form<UserNikeNameUpdate>,
) -> Result<Json<Value>> {
    let res = api::update_nike_name(service, user, user_nike_name_update).await?;
    Ok(Json(res))
}

//用户信息
#[handler]
pub async fn info(Data(service): Data<&Service>, Data(user): Data<&User>) -> Result<Json<Value>> {
    let res = api::info(service, user).await?;
    Ok(Json(res))
}

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

//创建用户
#[handler]
pub async fn create_by_code(
    Data(service): Data<&Service>,
    Form(form): Form<UserForm>,
) -> Result<Json<Value>> {
    let res = api::create_by_code(service, form).await?;
    Ok(Json(res))
}

pub fn new() -> impl Endpoint {
    let auth: Auth<Bearer> = Auth::new();
    let service_db = ServiceDb;

    Route::new()
        .at("/create", post(create))
        .at("/login", post(login))
        .at(
            "/info",
            get(info).with(service_db.clone()).with(auth.clone()),
        )
        .at(
            "/update_nike_name",
            post(update_nike_name)
                .with(service_db.clone())
                .with(auth.clone()),
        )
        .at("/create_by_code", post(create_by_code))
}
