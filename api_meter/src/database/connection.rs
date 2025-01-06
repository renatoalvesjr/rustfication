use sqlx::postgres::PgPoolOptions;

pub async fn conn() -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new()
        .max_connections(25)
        .connect("postgres://postgres:root@localhost/api_meter")
        .await
        .expect("Failed to connect to Postgres")
}

pub async fn db_health() -> u128 {

    let start = std::time::Instant::now();
    let pool = conn().await;
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Failed to execute query");
    let duration = start.elapsed().as_millis();
    duration
}