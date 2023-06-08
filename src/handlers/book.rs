use crate::models::book::{BookCreate, BookCreateData, BookCreateResponse, BookGet};

use axum::{Extension, Json};
use sqlx::PgPool;

pub async fn get_all_books(pool: Extension<PgPool>) -> Json<Vec<BookGet>> {
    let query = "SELECT title, author, owner_id FROM books";
    let books = sqlx::query_as::<_, BookGet>(query)
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch books from the database");

    Json(books)
}

pub async fn create_book(
    pool: Extension<PgPool>,
    Json(book): Json<BookCreate>,
) -> Json<BookCreateResponse> {
    let query = "INSERT INTO books(title, author, owner_id) VALUES ($1, $2, $3)";
    let res = sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.owner_id)
        .execute(&*pool)
        .await;

    let (status, message) = match res {
        Ok(_) => ("Success", "Book successfully added to the store!"),
        Err(_) => ("Failed", "Failed to add book to the store!"),
    };

    let response = BookCreateResponse {
        status: status.to_owned(),
        data: BookCreateData {
            message: message.to_owned(),
        },
    };

    Json(response)
}
