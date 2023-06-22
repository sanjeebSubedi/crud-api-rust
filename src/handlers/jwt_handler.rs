use anyhow::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8; 16] = b"V6Eew3btviYZWD0W";

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: String,
}

fn get_current_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to retrieve timestamp");

    duration.as_secs()
}

pub async fn sign_jwt(username: &str) -> Result<String, Error> {
    let payload = Claims {
        sub: username.to_string(),
        exp: get_current_timestamp() as usize + 3600,
    };
    let header = Header::default();
    Ok(jsonwebtoken::encode(
        &header,
        &payload,
        &EncodingKey::from_secret(JWT_SECRET),
    )?)
}

pub fn validate_jwt(token: &str) -> Result<(), Error> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(())
}
