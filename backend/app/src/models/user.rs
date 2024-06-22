use uuid::Uuid;

use crate::schemas::user::UserSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub avatar: Option<String>,
}
