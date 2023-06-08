use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    email: String,
    password: String,
    name: String,
    // address: String,
    // phone_number: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponseData {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub status: String,
    pub data: CreateUserResponseData,
}
