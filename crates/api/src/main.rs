use actix::{Actor, StreamHandler, ActorContext}; // 现在 ActorContext 在作用域内
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use db::{get_users, create_user, User};
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use actix_cors::Cors;

/// WebSocket Actor
struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// 处理 WebSocket 消息流
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

/// WebSocket 路由处理器
async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("请在 .env 文件中配置 DATABASE_URL");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("数据库连接失败，请检查 DATABASE_URL 配置");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(get_users_route))
            .route("/users", web::post().to(create_user_route))
            .route("/api/ws", web::get().to(ws_handler))
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_users_route(pool: web::Data<PgPool>) -> impl actix_web::Responder {
    let users = get_users(&pool).await.unwrap();
    actix_web::web::Json(users)
}

async fn create_user_route(
    pool: web::Data<PgPool>,
    user: actix_web::web::Json<User>,
) -> impl actix_web::Responder {
    create_user(&pool, &user).await.unwrap();
    "User added successfully"
}