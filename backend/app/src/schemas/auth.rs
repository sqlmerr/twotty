use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            exp: (Utc::now() + Duration::minutes(30)).timestamp() as usize,
        }
    }
}
