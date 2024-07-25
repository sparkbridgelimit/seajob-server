use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use listenfd::ListenFd;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: String,
}

#[derive(Debug, Clone)]
struct AppState {
}

async fn index() -> impl Responder {
    let response = ApiResponse {
        success: true,
        data: "hello rust man".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_url = "0.0.0.0:8080";
    let mut listenfd = ListenFd::from_env();

    // TODO
    let state = AppState {};

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}
