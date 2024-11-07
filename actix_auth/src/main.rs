use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{cookie, get, web, App, HttpResponse, HttpServer, Responder};
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
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new("Actix Web"))
            // unprotected routes
            .service(
                web::scope("/user")
                    .route("/login", web::post().to(login))
                    .route("/register", web::post().to(register))
                    //protected by login authentication
                    .route("/profile/{profile_id}", web::get().to(profile)), // .wrap(middleware::SayHi),
            )
            .service(hello)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
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

#[derive(serde::Deserialize)]
struct ProfileId {
    profile_id: String,
}

async fn profile(path: web::Path<ProfileId>) -> impl Responder {
    let auth_token =
        user_control::authentication(cookie::Cookie::named("auth-token").value().to_string()).await;
    println!("Profile request: {:?}", auth_token);
    match auth_token {
        true => HttpResponse::Ok().body(format!("Hello from Profile {}", path.profile_id)),
        false => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}

async fn login(login: web::Json<user_service::Login>) -> impl Responder {
    println!("Login request: {:?}", login);
    let token =
        user_control::login(String::from(&login.username), String::from(&login.password)).await;
    match token {
        Some(token) => HttpResponse::Ok()
            .cookie(cookie::Cookie::new("auth-token", token.clone()))
            .body(format!("Login Successful\n{}", &token)),

        None => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}

async fn register(user_register: web::Json<user_model::Model>) -> impl Responder {
    let register: user_model::Model = user_register.into_inner();
    HttpResponse::Ok().body(
        user_control::register(
            register.username,
            register.password,
            register.email,
            register.full_name,
        )
        .await
        .unwrap(),
    )
}
