use super::super::handlers::jwt_handler::validate_jwt;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn authenticate<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if validate_jwt(auth_header) => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
