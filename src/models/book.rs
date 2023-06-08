use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub owner_id: u32,
    // publication_year: u32,
    // description: String,
    // genre: String,
    // seller_id: u32,
    // listed_date: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct BookCreateData {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookCreateResponse {
    pub status: String,
    pub data: BookCreateData,
}
