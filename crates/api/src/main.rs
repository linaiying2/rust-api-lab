use actix_web::{web, App, HttpServer};
use db::create_connection_pool;
use log::LevelFilter;
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_connection_pool(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config_routes) // 使用配置函数
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}