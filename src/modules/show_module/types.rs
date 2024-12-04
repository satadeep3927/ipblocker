use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Entity {
    pub id: i32,
    pub ip: String,
    pub reason: String,
    pub timestamp: String,
}
