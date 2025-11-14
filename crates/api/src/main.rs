use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error, Responder};
use actix_web_actors::ws;
use actix_cors::Cors;
use db::{get_users, create_user, create_connection_pool, User};
use log::LevelFilter;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

// 保留路由模块化（从分支2继承，更易维护）
mod routes {
    use super::*;

    /// 配置所有路由（整合 CRUD + WebSocket）
    pub fn config_routes(cfg: &mut web::ServiceConfig) {
        cfg.route("/users", web::get().to(get_users_route))
           .route("/users", web::post().to(create_user_route))
           .route("/api/ws", web::get().to(ws_handler));
    }
}

/// WebSocket Actor（从分支1继承）
struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// WebSocket 消息处理（从分支1继承）
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

/// WebSocket 路由处理器（从分支1继承）
async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket {}, &req, stream)
}

/// 用户查询路由（从分支1继承，适配模块化）
async fn get_users_route(pool: web::Data<PgPool>) -> impl Responder {
    let users = get_users(&pool).await.expect("查询用户失败");
    web::Json(users)
}

/// 用户创建路由（从分支1继承，适配模块化）
async fn create_user_route(
    pool: web::Data<PgPool>,
    user: web::Json<User>,
) -> impl Responder {
    create_user(&pool, &user).await.expect("创建用户失败");
    "User added successfully"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 整合日志初始化（从分支2继承，便于调试）
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    // 整合环境变量加载（从分支1继承，兼容 .env 配置）
    dotenv().ok();

    // 数据库连接：使用分支2的连接池创建函数（更规范），保留分支1的错误提示
    let database_url = env::var("DATABASE_URL")
        .expect("请在 .env 文件中配置 DATABASE_URL（DATABASE_URL must be set）");
    let pool = create_connection_pool(&database_url)
        .await
        .expect("数据库连接失败，请检查 DATABASE_URL 配置（Failed to create pool）");

    // 保留启动提示（从分支2继承）
    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive()) // 保留跨域支持（从分支1继承）
            .configure(routes::config_routes) // 使用模块化路由配置
    })
    .bind("127.0.0.1:8080")? // 统一绑定格式（简化为标准写法）
    .run()
    .await
}