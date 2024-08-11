use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use log::info;
use seajob_common::response::ApiResponse;

use crate::AppState;

#[get("/")]
async fn index(req: HttpRequest, _data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    if let Some(user_id) = req.headers().get("X-User-Id") {
        if let Ok(user_id_str) = user_id.to_str() {
            info!("{:?}", user_id_str);
        }
    }
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}
