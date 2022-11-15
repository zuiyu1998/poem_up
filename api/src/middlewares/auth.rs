use std::marker::PhantomData;

use poem::{
    async_trait,
    http::header::{self, HeaderName},
    Endpoint, IntoResponse, Middleware, Request, Response,
};
use thiserror::Error;

use crate::error::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("HeaderValueNotFound")]
    HeaderValueNotFound,
    #[error("SchemaInvaild")]
    SchemaInvaild,
}

#[derive(Debug, Clone)]
pub struct Auth<S: Clone> {
    _marker: PhantomData<S>,
}

impl<S: Clone> Auth<S> {
    pub fn new() -> Self {
        Auth {
            _marker: PhantomData::default(),
        }
    }
}

impl<S: Schema, E: Endpoint> Middleware<E> for Auth<S> {
    type Output = AuthImpl<S, E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthImpl {
            _marker: PhantomData::default(),
            ep,
        }
    }
}

pub struct Token(pub String);

#[derive(Debug, Clone)]
pub struct Bearer;

impl Schema for Bearer {
    fn schema() -> &'static str {
        "Bearer"
    }
}

pub trait Schema: 'static + Send + Sync + Clone {
    fn header_name() -> HeaderName {
        header::AUTHORIZATION
    }

    fn schema() -> &'static str;

    fn parser(req: &Request) -> Result<Token, Error> {
        let header_value = req.header(Self::header_name());
        if header_value.is_none() {
            return Err(AuthError::HeaderValueNotFound.into());
        }
        let header_value = header_value.unwrap();
        let mut split_n = header_value.splitn(2, ' ');

        match split_n.next() {
            Some(schema) if schema == Self::schema() => {}
            _ => {
                return Err(AuthError::SchemaInvaild.into());
            }
        }

        match split_n.next() {
            Some(token) => {
                return Ok(Token(token.to_owned()));
            }
            _ => {
                return Err(AuthError::SchemaInvaild.into());
            }
        }
    }
}

pub struct AuthImpl<S, E> {
    _marker: PhantomData<S>,
    ep: E,
}

#[async_trait]
impl<S: Schema, E: Endpoint> Endpoint for AuthImpl<S, E> {
    type Output = Response;
    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let token = S::parser(&req)?;

        req.extensions_mut().insert(token);

        let res = self.ep.call(req).await?.into_response();
        Ok(res)
    }
}
