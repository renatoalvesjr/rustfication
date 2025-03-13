mod keys;
mod database;
mod errors;
mod auth;

use warp::Filter;
use warp::http::StatusCode;

#[tokio::main]
async fn main() {
    let health = warp::path("health")
        .and_then(|| async {
            let duration = database::connection::db_health().await;
            let status = format!("Server is running\t✅\nDatabase is running\t✅\nDatabase connection time: {} ms.", duration);
            Ok::<_, warp::Rejection>(warp::reply::with_status(status, StatusCode::OK))
        });
    let hello = warp::path("hello")
        .map(|| "Hello, World!");
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|user: database::queries::User| async {
            let pool = database::connection::conn().await;
            match database::queries::create_user(&pool, user.username, user.email, user.password).await {
                Ok(_) => Ok::<_, warp::Rejection>(warp::reply::with_status("User created successfully", StatusCode::CREATED)),
                Err(_) => Ok::<_, warp::Rejection>(warp::reply::with_status("Failed to create user", StatusCode::INTERNAL_SERVER_ERROR))
            }
        });

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|login: database::queries::Login| async  {
            match database::queries::user_login(login.email, login.password).await {
                Some(_) => Ok::<_, warp::Rejection>(warp::reply::with_status("Login successful", StatusCode::OK)),
                None => Ok::<_, warp::Rejection>(warp::reply::with_status("Login failed", StatusCode::UNAUTHORIZED))
            }
        });

    let routes = health
        .with(warp::cors().allow_any_origin())
        .or(hello)
        .or(register)
        .or(login);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    // database::queries::user_login("test2@email.com", "test2")
    //     .await
    //     .unwrap();
    //
    // database::queries::update_user_key("test2@email.com")
    //     .await
    //     .unwrap();
    //
    // database::queries::user_login("test2@email.com", "test2")
    //     .await
    //     .unwrap();
}
