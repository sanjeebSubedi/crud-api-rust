mod database;
mod handlers;
mod middleware;
mod models;
mod schemas;
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use handlers::{
    book::{create_book, get_all_books, get_books_by_user},
    token::get_token,
    user::{create_user, user_login},
};
use middleware::authenticate;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");
    let db = sqlx::postgres::PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    let app: Router = Router::new()
        .route("/users/books", get(get_books_by_user))
        .route("/books", post(create_book))
        .layer(from_fn(authenticate::authenticate))
        .route("/login", get(user_login))
        .route("/token", get(get_token))
        .route("/users", post(create_user))
        .route("/books", get(get_all_books))
        .layer(Extension(db));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
