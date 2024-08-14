use actix_web::{get, Error, HttpRequest, HttpResponse};
use seajob_common::response::ApiResponse;

// TODO: traefik中间件鉴权验证
#[get("/check")]
async fn check(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("x-user-id", 1))
        .json(ApiResponse::success_only()))
}
