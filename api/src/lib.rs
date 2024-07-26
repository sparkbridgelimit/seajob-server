mod router;
mod index;

use std::env;

use actix_web::{App, HttpServer, middleware, web};
use listenfd::ListenFd;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

// 服务主入口
#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // 环境配置获取
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let server_url = format!("{host}:{port}");

    // 连接数据库
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    // actix-web实例
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(router::config)
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
