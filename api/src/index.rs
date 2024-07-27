use actix_web::{Error, get, HttpRequest, HttpResponse, web};

use seajob_common::response::ApiResponse;

use crate::AppState;

#[get("/")]
async fn index(_req: HttpRequest, _data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}