use crate::models::book::BookAddData;

use super::super::models::book::{Book, BookAdd, BookAddResponse};
use axum::Json;
use sqlx::{PgPool, Row};

pub async fn get_all_books(pool: axum::extract::Extension<PgPool>) -> Json<Vec<Book>> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from the pool");
    let query = "SELECT title, author FROM BOOKS";
    let res = sqlx::query(query).fetch_all(&mut conn).await.unwrap();
    let books = res
        .iter()
        .map(|row| Book {
            title: row.get("title"),
            author: row.get("author"),
        })
        .collect();
    // println!("{:?}", res);
    Json(books)
}

pub async fn add_book(
    pool: axum::extract::Extension<PgPool>,
    Json(book): Json<BookAdd>,
) -> Json<BookAddResponse> {
    println!("{}", book.author);
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from the pool");

    let query = "INSERT INTO books(title, author, owner_id) VALUES ($1, $2, $3)";

    let res = sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.owner_id)
        .execute(&mut conn)
        .await;
    println!("{:?}", res);
    match res {
        Ok(_) => Json(BookAddResponse {
            status: "Success".to_owned(),
            data: BookAddData {
                message: "Book successfully added to the store!".to_owned(),
            },
        }),
        Err(_) => Json(BookAddResponse {
            status: "Failed".to_owned(),
            data: BookAddData {
                message: "Failed to add book to the store!".to_owned(),
            },
        }),
    }
}
