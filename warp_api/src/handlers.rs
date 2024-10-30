use super::models::Post;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    //Returning a static post
    let post = Post {
        id,
        title: String::from("Hello from Warp!"),
        body: String::from("This post was made from Warp."),
    };
    Ok(warp::reply::json(&post))
}
