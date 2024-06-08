use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::utils::{
    auth::decode_token,
    errors::{AppError, AuthError},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Username
    pub sub: String,
    /// Expiration
    pub exp: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl Claims {
    pub fn new(username: String) -> Self {
        Self {
            sub: username,
            exp: 2000000000,
        }
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        // Decode the user data
        let token_data = decode_token(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
