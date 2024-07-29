mod router;
mod index;
mod job_define;
mod job_contacted;
mod job_task;

use std::env;

use actix_web::{App, HttpServer, middleware, web};
use listenfd::ListenFd;
use env_logger::Env;
use seajob_common::db;
use seajob_service::entry::init_services;

#[derive(Debug, Clone)]
struct AppState {}

// 服务主入口
#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    init_services();
    db::init_db().await;

    let state = AppState {};

    // 环境配置获取
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{host}:{port}");

    // actix-web实例
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
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
