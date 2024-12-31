use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // This allows all origins
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
