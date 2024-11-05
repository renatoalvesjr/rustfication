use crate::encrypt::hash_func;
use crate::tokens::sign_token;

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String,
    pub email: String,
    pub full_name: String,
}

pub struct UserList {
    users: Vec<User>,
}

pub struct UserLogin {
    username: String,
    password: String,
}

pub async fn get_all_users() -> UserList {
    let user_list = UserList {
        users: vec![User {
            id: 1,
            username: "user1".to_string(),
            password: "password1".to_string(),
            email: "".to_string(),
            full_name: "User One".to_string(),
        }],
    };
    user_list
}

pub async fn get_user_by_username(username: &str) -> User {
    User {
        id: 1,
        username: username.to_string(),
        password: "password1".to_string(),
        email: "".to_string(),
        full_name: "User One".to_string(),
    }
}

pub async fn authenticate_user(user: UserLogin) -> Option<User> {
    let user_login = get_user_by_username(&user.username).await;

    if user_login.password == hash_func(&user.password) {
        sign_token(user_login.clone());
        Some(user_login)
    } else {
        None
    }
}
