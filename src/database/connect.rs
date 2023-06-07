use std::error::Error;

use sqlx::Pool;
// use sqlx::Row;

pub async fn connect() -> Result<Pool<sqlx::Postgres>, Box<dyn Error>> {
    let pool =
        sqlx::postgres::PgPool::connect("postgres://librarian:librarian@localhost:5432/library")
            .await?;
    Ok(pool)
}
