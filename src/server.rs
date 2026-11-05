use rocket::{get, serde::json::Json};

mod models;

#[get("/ping")]
pub fn ping()->Json<models::FSObject>{
    Json::from(models::FSObject::new(123, None, "xiao", models::FSObjectType::Dir))
}