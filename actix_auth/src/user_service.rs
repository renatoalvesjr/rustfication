use crate::encrypt::hash_func;
use crate::tokens::sign_token;

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
}

pub struct UserList {
    users: Vec<User>,
}

pub async fn get_all_users() -> UserList {
    let user_list = UserList {
        users: vec![User {
            id: 1,
            username: "user1".to_string(),
            password: hash_func("password1").to_string(),
            email: "email01@email.com".to_string(),
            full_name: "User One".to_string(),
        }],
    };
    user_list
}

pub async fn get_user_by_username(username: &str) -> Option<User> {
    let users = get_all_users().await;
    for user in users.users {
        if user.username == username {
            return Some(user);
        }
    }
    None
}

pub async fn get_user_by_email(email: &str) -> Option<User> {
    let users = get_all_users().await;
    for user in users.users {
        if user.email == email {
            return Some(user);
        }
    }
    None
}

pub async fn authenticate_user(username: String, password: String) -> Option<String> {
    let mut user_login = get_user_by_username(&username).await;
    if user_login.is_none() {
        user_login = get_user_by_email(&username).await;
    }

    match user_login {
        Some(user) => {
            if hash_func(&password) == user.password {
                Some(sign_token(user).unwrap())
            } else {
                None
            }
        }
        None => None,
    }
}
