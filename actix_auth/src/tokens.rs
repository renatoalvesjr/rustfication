use crate::user_model::Model as User;
use actix_web::Error;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: i32,
    username: String,
    email: String,
    exp: usize,
}

pub fn sign_token(user: User) -> Result<String, Error> {
    let expiration = (Utc::now() + chrono::Duration::hours(2)).timestamp() as usize;
    let header: Header = Header::new(Algorithm::HS512);
    let claims: Claims = Claims {
        id: user.id,
        username: user.username,
        email: user.email,
        exp: expiration,
    };
    let key: EncodingKey = EncodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes());
    let token = encode(&header, &claims, &key)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(token)
}

pub fn token_valid(token: String) -> bool {
    let token: Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> =
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
    match token {
        Ok(_) => true,
        Err(_) => false,
    }
}
