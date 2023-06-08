use axum::{Extension, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    models::user::User,
    schemas::user::{
        CreateUser, CreateUserResponse, CreateUserResponseData, LoginResponse, LoginResponseData,
        LoginUser,
    },
};
use bcrypt::{hash, verify, DEFAULT_COST};

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

pub fn login_response(status: &str, message: &str) -> Json<LoginResponse> {
    Json(LoginResponse {
        status: status.to_owned(),
        data: LoginResponseData {
            message: message.to_owned(),
        },
    })
}

pub async fn user_login(
    pool: Extension<PgPool>,
    Json(credentials): Json<LoginUser>,
) -> Json<LoginResponse> {
    if credentials.validate().is_err() {
        return login_response("Failed!", "Validation failed!");
    }
    let query = "SELECT * FROM users WHERE email = $1";
    let res = sqlx::query_as::<_, User>(query)
        .bind(credentials.email)
        .fetch_optional(&*pool)
        .await;

    let (status, message) = match res {
        Ok(Some(data)) => {
            let valid = verify(&credentials.password, &data.password);
            match valid {
                Ok(true) => ("Success", "Login Successful"),
                Ok(false) => ("Failed", "Incorrect Password"),
                Err(_) => ("Failed", "Password verification error!"),
            }
        }
        Ok(None) => ("Failed", "Email not found!"),
        Err(_) => ("Failed", "Failed to connect to the database!"),
    };

    login_response(status, message)
}
