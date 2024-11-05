use crate::user_service::User;
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn sign_token(user: User) -> String {
    dotenv().ok();
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(std::env::var("TOKEN_SECRET").unwrap().as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("username", user.username);
    claims.insert("email", user.email);
    let token = claims.sign_with_key(&key).unwrap();
    token
}

pub fn token_valid(token: String) -> bool {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(std::env::var("TOKEN_SECRET").unwrap().as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).unwrap();

    if claims.contains_key("username") && claims.contains_key("email") {
        true
    } else {
        false
    }
}
