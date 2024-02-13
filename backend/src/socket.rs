use actix_web::{get, Responder};

#[get("/socket")]
pub async fn socket() -> impl Responder {
  "Hello, world!"
}