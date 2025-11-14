use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<i32>) -> impl Responder {
    let user = User { id: path.into_inner(), name: "Alice".into() };
    HttpResponse::Ok().json(user)
}

#[post("/user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.0)
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}