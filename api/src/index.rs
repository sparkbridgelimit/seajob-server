use actix_web::{get, web, Error, HttpRequest, HttpResponse, HttpMessage};
use log::info;
use seajob_common::response::ApiResponse;
use seajob_dto::user_context::UserContext;
use crate::AppState;

#[get("/")]
async fn index(req: HttpRequest, _data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // 尝试从请求的 extensions 中获取 UserContext
    if let Some(user_context) = req.extensions().get::<UserContext>() {
        // 打印 user_id
        info!("User ID: {:?}", user_context.user_id);
    } else {
        // 如果没有 UserContext
        info!("No UserContext found");
    }
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}
