use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
