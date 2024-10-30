use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}
