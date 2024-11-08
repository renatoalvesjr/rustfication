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

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/user")
                    .route("/login", web::post().to(login))
                    .route("/register", web::post().to(register)), // .route("/profile/{profile_id}", web::get().to(profile)),
            )
            .service(hello)
            .default_service(web::route().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello from /");
    HttpResponse::Ok().body(format!("Hello from /"))
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Wrong route sailor!")
}

// #[derive(serde::Deserialize)]
// struct ProfileId {
//     profile_id: String,
// }

// async fn profile(path: web::Path<String>) -> impl Responder {
//     println!("Authenticating profile request");
//     let cookie = cookie::Cookie::println!("Cookie: {:?}", cookie);
//     // let session = user_control::authentication(cookie).await;
//     let session = true;
//     println!("Authenticated: {}", session);
//     match session {
//         true => HttpResponse::Ok().body(format!("Hello from Profile {}", path)),
//         false => HttpResponse::Unauthorized().body("Unauthorized"),
//     }
// }

async fn login(login: web::Json<user_service::Login>) -> impl Responder {
    let token =
        user_control::login(String::from(&login.username), String::from(&login.password)).await;
    match token {
        Some(token) => HttpResponse::Ok()
            .cookie(cookie::Cookie::new("session", token.clone()))
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
