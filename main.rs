mod routes;
mod db;
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
        let _pool = db::connect().await.expect("Failed to connect to database");
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let addr = format!("127.0.0.1:{}", port);
        info!("🚀 Server starting at http://{}", addr);

    HttpServer::new(|| {
        App::new().configure(routes::register)
    })
    .bind(addr)?
    .run()
    .await
}
