use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    // email: String,
    password: String,
    name: String,
    // address: String,
    // phone_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub status: String,
    pub data: String,
}
