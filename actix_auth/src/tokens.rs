use crate::user_service::User;
use actix_session::Session;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::HashMap;

pub fn sign_token(user: User) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = HashMap::new();
    claims.insert("username", user.username);
    claims.insert("email", user.email);

    let token = claims.sign_with_key(&key).unwrap();
    token
}

pub fn token_valid(token: String) -> bool {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let claims: HashMap<String, String> = token.verify_with_key(&key).unwrap();

    if claims.contains_key("username") && claims.contains_key("email") {
        true
    } else {
        false
    }
}
