use super::super::handlers::jwt_handler::validate_jwt;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

pub async fn authenticate<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) => match validate_jwt(auth_header) {
            Ok(_) => Ok(next.run(request).await),
            Err(e) => Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"status": "failed", "message": e.to_string()})),
            )),
        },
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"status": "failed", "message": "Auth header not found."})),
        )),
    }
}
