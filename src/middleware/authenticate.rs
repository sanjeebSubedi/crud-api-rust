use super::super::handlers::jwt_handler::validate_jwt;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct AuthResponseData {
    message: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    status: String,
    data: AuthResponseData,
}

pub async fn authenticate<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<AuthResponse>)> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        // Some(auth_header) if validate_jwt(auth_header).is_ok() => Ok(next.run(request).await),
        Some(auth_header) => match validate_jwt(auth_header) {
            Ok(_) => Ok(next.run(request).await),
            Err(e) => {
                let res = AuthResponse {
                    status: "failed".to_string(),
                    data: AuthResponseData {
                        message: e.to_string(),
                    },
                };
                Err((StatusCode::UNAUTHORIZED, Json(res)))
            }
        },
        _ => {
            let res = AuthResponse {
                status: "failed".to_string(),
                data: AuthResponseData {
                    message: "Auth header not found".to_string(),
                },
            };
            Err((StatusCode::UNAUTHORIZED, Json(res)))
        }
    }
}
