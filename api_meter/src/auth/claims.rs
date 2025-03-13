use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
    pub exp: usize,
}

pub fn generate_claims(username: String, email: String) -> Claims {
    Claims {
        username,
        email,
        exp: 10000000000,
    }
}

pub fn validate_claims(claims: &Claims) -> bool {
    let now = chrono::Utc::now().timestamp() as usize;
    claims.exp > now
}

pub fn get_username(claims: &Claims) -> String {
    claims.username.clone()
}

pub fn get_email(claims: &Claims) -> String {
    claims.email.clone()
}

pub fn get_exp(claims: &Claims) -> usize {
    claims.exp
}
