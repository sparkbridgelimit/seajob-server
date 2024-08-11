use actix_web::{Error, get, HttpRequest, HttpResponse};
use seajob_common::response::ApiResponse;

// TODO: traefik中间件鉴权验证
#[get("/")]
async fn check(_req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("X-User-Id", 1))
        .json(ApiResponse::success_only())
    )
}