use actix_web::{get, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "i am the paradox"
}

pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(hello);
}
