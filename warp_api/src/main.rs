mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    println!("Starting server at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
