use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    // pub id: u32,
    pub title: String,
    pub author: String,
    // pub owner_id: u32,
    // publication_year: u32,
    // description: String,
    // genre: String,
    // seller_id: u32,
    // listed_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookAdd {
    pub title: String,
    pub author: String,
    pub owner_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookAddData {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookAddResponse {
    pub status: String,
    pub data: BookAddData,
}
