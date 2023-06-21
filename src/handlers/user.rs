use super::jwt_handler::sign_jwt;
use crate::{
    models::user::User,
    schemas::user::{
        CreateUser, CreateUserResponse, CreateUserResponseData, LoginResponse, LoginResponseData,
        LoginUser,
    },
};
use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use validator::Validate;

pub fn return_response(status: &str, access_token: &str) -> Json<CreateUserResponse> {
    Json(CreateUserResponse {
        status: status.to_owned(),
        data: CreateUserResponseData {
            access_token: access_token.to_owned(),
        },
    })
}

pub async fn create_user(
    pool: axum::extract::Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> (StatusCode, Json<CreateUserResponse>) {
    if user.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            return_response("Failed!", "Validation failed!"),
        );
    }

    let hashed = match hash(user.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                return_response("Failed", "Failed to hash the password"),
            )
        }
    };

    let query = "INSERT INTO users(email, password, name) VALUES ($1, $2, $3)";

    let res = sqlx::query(query)
        .bind(&user.email)
        .bind(hashed)
        .bind(user.name)
        .execute(&*pool)
        .await;
    if res.is_err() {
        (
            StatusCode::BAD_REQUEST,
            return_response("Failed", "Failed to create new user"),
        )
    } else if let Ok(token) = sign_jwt(&user.email).await {
        (StatusCode::OK, return_response("Success", &token))
    } else {
        (
            StatusCode::BAD_REQUEST,
            return_response("Failed", "Failed to create new user"),
        )
    }
}

pub fn login_response(status: &str, access_token: &str) -> Json<LoginResponse> {
    Json(LoginResponse {
        status: status.to_owned(),
        data: LoginResponseData {
            access_token: access_token.to_owned(),
        },
    })
}

pub async fn user_login(
    pool: Extension<PgPool>,
    Json(credentials): Json<LoginUser>,
) -> (StatusCode, Json<LoginResponse>) {
    if credentials.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            login_response("Failed!", "Validation failed!"),
        );
    }
    let query = "SELECT * FROM users WHERE email = $1";
    let res = sqlx::query_as::<_, User>(query)
        .bind(&credentials.email)
        .fetch_optional(&*pool)
        .await;

    let (status_code, status, message) = match res {
        Ok(Some(data)) => {
            let valid = verify(&credentials.password, &data.password);
            match valid {
                Ok(true) => (StatusCode::OK, "Success", "Login Successful"),
                Ok(false) => (StatusCode::UNAUTHORIZED, "Failed", "Incorrect Password"),
                Err(_) => (
                    StatusCode::UNAUTHORIZED,
                    "Failed",
                    "Password verification error!",
                ),
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "Failed", "Email not found!"),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            "Failed",
            "Failed to connect to the database!",
        ),
    };
    if status_code == StatusCode::OK {
        if let Ok(token) = sign_jwt(&credentials.email).await {
            return (status_code, login_response(status, &token));
        }
    }
    (status_code, login_response(status, message))
}
