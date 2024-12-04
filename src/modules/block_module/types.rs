use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub ip: String,
    pub reason: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
