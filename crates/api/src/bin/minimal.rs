use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello from Rust API!"
}

#[derive(Deserialize)]
struct MetricReq {
    endpoint: String,
    response_time_ms: i32,
}

#[post("/metric")]
async fn push_metric(req: web::Json<MetricReq>) -> impl Responder {
    println!(
        "✅ Received metric: endpoint={}, response_time_ms={}",
        req.endpoint, req.response_time_ms
    );
    format!(
        "Received metric from {} with response_time_ms={}",
        req.endpoint, req.response_time_ms
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080 ...");
    HttpServer::new(|| App::new().service(hello).service(push_metric))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
