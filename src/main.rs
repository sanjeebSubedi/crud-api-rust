mod database;
mod handlers;
mod models;
mod schemas;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use handlers::{
    book::{create_book, get_all_books, get_books_by_user},
    user::{create_user, user_login},
};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
    let db = sqlx::postgres::PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    let app: Router = Router::new()
        .route("/books", get(get_all_books))
        .route("/books", post(create_book))
        .route("/users", post(create_user))
        .route("/users/books", get(get_books_by_user))
        .route("/login", get(user_login))
        .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
