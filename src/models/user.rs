use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    id: i64,
    email: String,
    pub password: String,
    name: String,
    // address: String,
    // phone_number: String,
}
