use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::env;

pub mod database;
pub mod encrypt;
pub mod middleware;
pub mod tokens;
pub mod user_control;
pub mod user_model;
pub mod user_service;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new("Actix Web"))
            // unprotected routes
            .service(
                web::scope("/user")
                    .route("/login", web::get().to(login))
                    .route("/register", web::get().to(register))
                    //protected by login authentication
                    .route("/profile/{profile_id}", web::get().to(profile)),
            )
            .service(hello)
    })
    .bind((host, port))?
    .run()
    .await
}

#[get("/")]
async fn hello(data: web::Data<&str>) -> impl Responder {
    let app_data = &data;
    println!(
        "Hello from {:?}, {}",
        app_data,
        env::var("DATABASE_URL").unwrap()
    );
    HttpResponse::Ok().body(format!(
        "Hello from {:?}, {}",
        app_data,
        env::var("DATABASE_URL").unwrap()
    ))
}

#[derive(Deserialize)]
struct Info {
    profile_id: String,
}

async fn profile(path: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from Profile {}", path.profile_id))
}

async fn login() -> impl Responder {
    let token = user_control::login(String::from("test@email.com"), String::from("test123")).await;
    match token {
        Some(token) => HttpResponse::Ok().body(token),
        None => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}

async fn register() -> impl Responder {
    HttpResponse::Ok().body("Hello from Register")
}
