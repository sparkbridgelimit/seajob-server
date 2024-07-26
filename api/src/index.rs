use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use seajob_common::response::ApiResponse;
use seajob_service::contacted_job_service::ContactedJobService;
use crate::AppState;

#[get("/")]
async fn index(_req: HttpRequest, _data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}

#[get("/contacted")]
async fn find_contacted_job(_req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let contacted_jobs = ContactedJobService::find_all(conn).await.expect("Cannot find ContactedJob");

    let response = ApiResponse::success(contacted_jobs);
    Ok(HttpResponse::Ok().json(response))
}