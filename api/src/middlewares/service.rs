use entity::{
    sea_orm::Set,
    users::{ActiveModel, Model},
};
use hmac::{Hmac, Mac};
use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response};
use sha2::Sha256;

use poem_up_service::Service;
use thiserror::Error;
use tracing::info;

use crate::error::Result;

use super::Token;

#[derive(Debug, Error)]
pub enum ServiceDbError {
    #[error("TokenNotFound")]
    TokenNotFound,
    #[error("ServiceNotFound")]
    ServiceNotFound,
    #[error("InvalidKey")]
    InvalidKey,
    #[error("SignFailed")]
    SignFailed,
    #[error("ParseFailed")]
    ParseFailed,
    #[error("MissingSubject")]
    MissingSubject,
}

#[derive(Debug, Clone)]
pub struct ServiceDb;

impl<E: Endpoint> Middleware<E> for ServiceDb {
    type Output = ServiceDbImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ServiceDbImpl { ep }
    }
}

pub struct ServiceDbImpl<E> {
    ep: E,
}

impl<E> ServiceDbImpl<E> {
    pub async fn handle(&self, req: &mut Request) -> Result<Model> {
        let token = req
            .extensions()
            .get::<Token>()
            .ok_or(ServiceDbError::TokenNotFound)?;
        let service = req
            .extensions()
            .get::<Service>()
            .ok_or(ServiceDbError::ServiceNotFound)?;
        let uid = decode(&token.0)?;

        let mut active = ActiveModel::default();
        active.uid = Set(uid);

        let transaction = service.transaction().await?;

        let user_service = transaction.user();
        let user = user_service.find(&active).await?;
        transaction.commit().await?;

        Ok(user)
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for ServiceDbImpl<E> {
    type Output = Response;
    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let user = self.handle(&mut req).await?;

        req.extensions_mut().insert(user);

        let res = self.ep.call(req).await?.into_response();
        Ok(res)
    }
}

pub fn encode(raw: &str) -> Result<String> {
    info!("encode raw str: {:?}", raw);

    let claims = RegisteredClaims {
        issuer: Some("up.com".into()),
        subject: Some(raw.into()),
        ..Default::default()
    };
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(b"poem_up_secret_key").map_err(|_e| ServiceDbError::InvalidKey)?;

    let signed_token = claims
        .sign_with_key(&key)
        .map_err(|_e| ServiceDbError::SignFailed)?;
    Ok(signed_token)
}

pub fn decode(token: &str) -> Result<String> {
    info!("decode token : {:?}", token);

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(b"poem_up_secret_key").map_err(|_e| ServiceDbError::InvalidKey)?;
    let claims: RegisteredClaims =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| ServiceDbError::ParseFailed)?;

    claims.subject.ok_or(ServiceDbError::MissingSubject.into())
}
