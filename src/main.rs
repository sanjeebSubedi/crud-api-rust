mod database;
mod handlers;
mod models;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use handlers::{
    book::{add_book, get_all_books},
    user::create_user,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = database::connect::connect().await?;
    // sqlx::migrate!("./migrations").run(&db).await?;
    let app: Router = Router::new()
        .route("/", get(landing_page_handler))
        .route("/books", get(get_all_books))
        .route("/books", post(add_book))
        .route("/users", post(create_user))
        .layer(Extension(db.clone()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn landing_page_handler() -> String {
    "Hello everyone".to_owned()
}
