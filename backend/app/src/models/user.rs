use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }
}
