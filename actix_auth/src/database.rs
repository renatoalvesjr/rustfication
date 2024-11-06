use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn conn() -> DatabaseConnection {
    Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to the database")
}
