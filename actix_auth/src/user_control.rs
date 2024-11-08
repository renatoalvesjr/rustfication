use crate::tokens;
use crate::user_service::{self, authenticate_user};

pub async fn login(login: String, password: String) -> Option<String> {
    let token: Option<String> = authenticate_user(login, password).await;
    match token {
        Some(token) => Some(token),
        None => None,
    }
}

pub async fn authentication(token: String) -> bool {
    tokens::token_valid(token)
}

pub async fn register(
    username: String,
    password: String,
    email: String,
    full_name: Option<String>,
) -> Option<String> {
     user_service::register(username, password, email, full_name).await
}
