use axum::Json;
use sqlx::PgPool;
use validator::Validate;

use crate::models::user::{CreateUser, CreateUserResponse, CreateUserResponseData};
use bcrypt::{hash, DEFAULT_COST};

pub fn return_response(status: &str, message: &str) -> Json<CreateUserResponse> {
    Json(CreateUserResponse {
        status: status.to_owned(),
        data: CreateUserResponseData {
            message: message.to_owned(),
        },
    })
}

pub async fn create_user(
    pool: axum::extract::Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> Json<CreateUserResponse> {
    if user.validate().is_err() {
        return return_response("Failed!", "Validation failed!");
    }

    let hashed = match hash(user.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return return_response("Failed", "Failed to hash the password"),
    };

    let query = "INSERT INTO users(email, password, name) VALUES ($1, $2, $3)";

    let res = sqlx::query(query)
        .bind(&user.email)
        .bind(hashed)
        .bind(user.name)
        .execute(&*pool)
        .await;
    if res.is_err() {
        return_response("Failed", "Failed to create new user")
    } else {
        return_response("Success", "New user was successfully created!")
    }
}
