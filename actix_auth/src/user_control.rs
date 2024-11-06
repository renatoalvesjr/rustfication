use crate::tokens;
use crate::user_service::authenticate_user;

pub async fn login(login: String, password: String) -> Option<String> {
    let token = authenticate_user(login, password).await;
    match token {
        Some(token) => Some(token),
        None => None,
    }
}

pub async fn authentication(token: String) -> bool {
    tokens::token_valid(token)
}
