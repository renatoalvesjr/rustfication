use crate::database::connection::conn;
use crate::keys;
use argon2::password_hash::rand_core::OsRng;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::Deserialize;
use sqlx::{query, Executor, Row};

#[derive(Deserialize)]
pub(crate) struct Login {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub key: String,
    pub creation_date: String, // Change to String
}

fn hash_pass(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    password_hash
}
pub async fn create_user(
    pool: &sqlx::Pool<sqlx::Postgres>,
    username: String,
    email: String,
    password: String,
) -> Result<(), sqlx::Error> {
    const CREATE_USER_QUERY: &str =
        "INSERT INTO users (username, email, password, key) VALUES ($1, $2, $3, $4)";

    sqlx::query(CREATE_USER_QUERY)
        .bind(username)
        .bind(email)
        .bind(hash_pass(&password))
        .bind(keys::generate_key())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn user_login(
    email: String,
    password: String,
) -> Option<Login>
{
    const USER_LOGIN_QUERY: &str = "SELECT * FROM users WHERE email = $1";

    let pool = conn().await;
    let row = query(USER_LOGIN_QUERY)
        .bind(email)
        .fetch_one(&pool)
        .await
        .ok();
    match row {
        Some(row) => {
            let login: Login = Login {
                email: row.get("email"),
                password: row.get("password"),
            };

            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&login.password).unwrap();
            let is_valid = argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();
            if is_valid {
                println!("Valid password");
                println!("Username: {}", login.email);

                Some(login)
            } else {
                println!("Invalid password");
                None
            }
        }
        None => None,
    }
}

/// Updates the API key for a user identified by their email.
///
/// # Arguments
///
/// * `email` - A string slice that holds the email of the user.
///
/// # Returns
///
/// * `Result<(), sqlx::Error>` - Returns an empty result on success, or a `sqlx::Error` on failure.
///
/// # Example
///
/// ```rust
/// use crate::database::queries::update_user_key;
///
/// let result = update_user_key("user@example.com").await;
/// match result {
///     Ok(_) => println!("User key updated successfully"),
///     Err(e) => eprintln!("Failed to update user key: {}", e),
/// }
pub async fn update_user_key(email: &str) -> Result<(), sqlx::Error> {
    const UPDATE_USER_KEY_QUERY: &str = "UPDATE users SET key = $1 WHERE email = $2";

    let pool = conn().await;
    let new_key = keys::generate_key();
    query(UPDATE_USER_KEY_QUERY)
        .bind(new_key)
        .bind(email)
        .execute(&pool)
        .await?;
    Ok(())
}
