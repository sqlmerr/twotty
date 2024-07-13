use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Following {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
}
