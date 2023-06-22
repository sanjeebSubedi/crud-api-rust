use crate::schemas::book::{BookCreate, BookGet};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct OwnerId {
    pub owner_id: i32,
}

pub async fn get_books_by_user(
    pool: Extension<PgPool>,
    Json(data): Json<OwnerId>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = "SELECT title, author, owner_id from books where owner_id = $1";
    match sqlx::query_as::<_, BookGet>(query)
        .bind(data.owner_id)
        .fetch_all(&*pool)
        .await
    {
        Ok(books) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "data": books})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({"status": "failed", "message": "Error while connecting to the database!"}),
            ),
        )),
    }
}

pub async fn get_all_books(
    pool: Extension<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = "SELECT title, author, owner_id FROM books";
    match sqlx::query_as::<_, BookGet>(query).fetch_all(&*pool).await {
        Ok(books) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"status": "success", "data": books})),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({"status": "failed", "message": "Error while connecting to the database!"}),
            ),
        )),
    }
}

pub async fn create_book(
    pool: Extension<PgPool>,
    Json(book): Json<BookCreate>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = "INSERT INTO books(title, author, owner_id) VALUES ($1, $2, $3)";
    let res = sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.owner_id)
        .execute(&*pool)
        .await;

    match res {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(
                serde_json::json!({"status": "success", "message": "Book successfully added to the store."}),
            ),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({"status": "failed", "message": "Failed to add book to the store"}),
            ),
        )),
    }
}
