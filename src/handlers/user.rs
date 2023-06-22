use super::jwt_handler::sign_jwt;
use crate::{
    models::user::User,
    schemas::user::{CreateUser, LoginUser},
};
use axum::{http::StatusCode, response::IntoResponse};
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_user(
    pool: axum::extract::Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if user.validate().is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "failed", "message": "User validation failed!"})),
        ));
    }

    let hashed = match hash(user.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"status": "failed", "message": "Failed to hash the password!"}),
                ),
            ))
        }
    };

    let query = "INSERT INTO users(email, password, name) VALUES ($1, $2, $3)";

    let res = sqlx::query(query)
        .bind(&user.email)
        .bind(hashed)
        .bind(user.name)
        .execute(&*pool)
        .await;

    match (res, sign_jwt(&user.email).await) {
        (Ok(_), Ok(token)) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "data": {"token": token}})),
        )),
        (_, _) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "failed", "message": "failed to create new user"})),
        )),
    }
}

pub async fn user_login(
    pool: Extension<PgPool>,
    Json(credentials): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if credentials.validate().is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "failed", "message": "Validation Error"})),
        ));
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
                Ok(true) => (StatusCode::OK, "success", "Login Successful"),
                Ok(false) => (StatusCode::UNAUTHORIZED, "failed", "Incorrect Password"),
                Err(_) => (
                    StatusCode::UNAUTHORIZED,
                    "failed",
                    "Password verification error!",
                ),
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "failed", "Email not found!"),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed",
            "Failed to connect to the database!",
        ),
    };
    if status_code == StatusCode::OK {
        if let Ok(token) = sign_jwt(&credentials.email).await {
            return Ok((
                status_code,
                Json(serde_json::json!({"status": "success", "data": {"token": token}})),
            ));
        }
    }
    Err((
        status_code,
        Json(serde_json::json!({"status": status, "message": message})),
    ))
}
