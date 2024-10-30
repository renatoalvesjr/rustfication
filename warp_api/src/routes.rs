use super::handlers;
use warp::Filter;

//Route builder function
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post()
}

fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("posts")
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and(warp::get())
        .and_then(handlers::get_post)
}
