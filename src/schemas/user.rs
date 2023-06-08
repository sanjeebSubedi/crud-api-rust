use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponseData {
    pub message: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub status: String,
    pub data: CreateUserResponseData,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseData {
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: String,
    pub data: LoginResponseData,
}
