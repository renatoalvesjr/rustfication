use crate::user_service::{authenticate_user, get_all_users, get_user_by_username, UserLogin};
use actix_web::{get, web, HttpResponse, Responder};

async fn login(user_login: UserLogin) {
    let user = authenticate_user(user_login).await;
}
