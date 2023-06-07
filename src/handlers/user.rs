use axum::Json;
use sqlx::PgPool;

use crate::models::user::{CreateUser, CreateUserResponse};

pub async fn create_user(
    pool: axum::extract::Extension<PgPool>,
    Json(user): Json<CreateUser>,
) -> Json<CreateUserResponse> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from the pool");

    let query = "INSERT INTO users(username, password, name) VALUES ($1, $2, $3)";

    let res = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.password)
        .bind(user.name)
        .execute(&mut conn)
        .await;
    println!("{:?}", res);
    match res {
        Ok(_) => Json(CreateUserResponse {
            status: "Success".to_owned(),
            data: "New user was successfully created!".to_owned(),
        }),
        Err(_) => Json(CreateUserResponse {
            status: "Failed".to_owned(),
            data: "Failed to create new user!".to_owned(),
        }),
    }
}
