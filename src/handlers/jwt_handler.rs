use axum::Json;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8; 16] = b"V6Eew3btviYZWD0W";

#[derive(Debug)]
pub struct TokenResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn sign_jwt(username: &str) -> Json<TokenResponse> {
    let payload = Claims {
        sub: username.to_string(),
        exp: get_current_timestamp() as usize + 3600,
    };
    let header = Header::default();
    match jsonwebtoken::encode(&header, &payload, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(token) => Json(TokenResponse {
            access_token: token,
        }),
        Err(_) => Json(TokenResponse {
            access_token: "at".to_string(),
        }),
    }
}

pub fn validate_jwt(token: &str) -> bool {
    let decoded_token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    );
    let decoded_token = match decoded_token {
        Ok(token) => token,
        Err(_) => return false,
    };
    let ts_now = SystemTime::now().duration_since(UNIX_EPOCH);
    match ts_now {
        Ok(ts) => decoded_token.claims.exp > ts.as_secs() as usize,
        Err(_) => false,
    }
}
