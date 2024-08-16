use std::env;

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::{middleware, web, App, HttpMessage, HttpServer};
use env_logger::Env;
use listenfd::ListenFd;

use seajob_common::{db, redis_client};
use seajob_dto::user_context::UserContext;
use seajob_service::entry::init_services;

mod index;
mod router;

#[derive(Debug, Clone)]
struct AppState {}

// 服务主入口
#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    init_services();

    // 初始化DB连接
    db::init_db().await;
    // 初始化redis连接
    redis_client::init_redis().await;

    let state = AppState {};

    // 环境配置获取
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{host}:{port}");

    // actix-web实例
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|_origin, _req_head| true) // 支持任何来源
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::HeaderName::from_static("x-user-id"),
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .wrap_fn(|req, srv| {
                if let Some(user_id) = req.headers().get("x-user-id") {
                    if let Ok(user_id_str) = user_id.to_str() {
                        // 将 user_id 插入到请求的 extensions 中
                        req.extensions_mut().insert(UserContext {
                            user_id: user_id_str.parse::<i64>().unwrap(),
                        });
                    }
                }
                srv.call(req)
            })
            .configure(router::entry)
    });

    // 绑定文件描述符, 使得重启后可以继续连接原来的tcp
    let mut listenfd = ListenFd::from_env();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        print!("Error: {err}");
    }
}