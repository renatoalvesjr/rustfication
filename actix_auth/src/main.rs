use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod encrypt;
pub mod tokens;
pub mod user_control;
pub mod user_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 3344;
    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new("Actix Web"))
            .service(
                web::scope("/user")
                    .route("/login", web::get().to(login))
                    .route("/register", web::get().to(register)),
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
    println!("Hello from {:?}", app_data);
    HttpResponse::Ok().body(format!("Hello from {:?}", app_data))
}

async fn login() -> impl Responder {
    let token =
        user_control::login(String::from("emai1@email.com"), String::from("password1")).await;
    match token {
        Some(token) => HttpResponse::Ok().body(token),
        None => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}

async fn register() -> impl Responder {
    HttpResponse::Ok().body("Hello from Register")
}
