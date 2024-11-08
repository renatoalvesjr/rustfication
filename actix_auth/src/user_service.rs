use std::env;

use crate::database::conn;
use crate::encrypt::{password_hash, password_verify};
use crate::tokens::sign_token;
use crate::user_model::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(serde::Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

async fn user_exists(username: &str, email: &str) -> bool {
    get_user_by_username(username).await.is_some() || get_user_by_email(email).await.is_some()
}

async fn get_user_by_username(username: &str) -> Option<Model> {
    match Entity::find()
        .filter(Column::Username.contains(username))
        .one(&conn().await)
        .await
    {
        Ok(user) => Some(user)?,
        Err(_) => None,
    }
}

async fn get_user_by_email(email: &str) -> Option<Model> {
    match Entity::find()
        .filter(Column::Email.contains(email))
        .one(&conn().await)
        .await
    {
        Ok(user) => Some(user)?,
        Err(_) => None,
    }
}

pub async fn authenticate_user(username: String, password: String) -> Option<String> {
    let mut user_login = get_user_by_username(&username).await;
    if user_login.is_none() {
        user_login = get_user_by_email(&username).await;
    }

    match user_login {
        Some(user) => {
            if password_verify(
                format!("{}{}", env::var("SALT").unwrap_or_default(), &password).as_str(),
                &user.password,
            ) {
                return Some(sign_token(user).unwrap());
            }
            None
        }
        None => None,
    }
}

pub async fn register(
    username: String,
    password: String,
    email: String,
    full_name: Option<String>,
) -> Option<String> {
    let new_user: ActiveModel = ActiveModel {
        id: NotSet,
        username: Set(username.clone()),
        password: Set(password_hash(
            format!("{}{}", env::var("SALT").unwrap_or_default(), &password).as_str(),
        )),
        email: Set(email.clone()),
        full_name: Set(full_name.clone()),
    };
    if user_exists(&username, &email).await {
        return Some("User already exists".to_string());
    }
    Entity::insert(new_user).exec(&conn().await).await.unwrap();
    Some("User registered".to_string())
}

pub async fn change_full_name(new_name: &str, user_id: i32) -> Result<(), sea_orm::error::DbErr> {
    let db = conn().await;
    let user: Option<ActiveModel> = Entity::find_by_id(user_id).one(&db).await?.map(Into::into);

    if let Some(mut user) = user {
        user.full_name = Set(Some(new_name.to_string()));
        user.update(&db).await?;
    }

    Ok(())
}
