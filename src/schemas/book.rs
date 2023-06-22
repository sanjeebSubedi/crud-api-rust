use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookCreate {
    pub title: String,
    pub author: String,
    pub owner_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BookGet {
    pub title: String,
    pub author: String,
    pub owner_id: i64,
}
