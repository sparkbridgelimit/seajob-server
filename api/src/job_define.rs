use actix_web::{delete, Error, get, HttpRequest, HttpResponse, post, put, web};
use seajob_common::response::{ApiResponse};


use crate::AppState;
use seajob_service::entry::JOB_DEFINE_SERVICE;
use log::error;

// TODO: 获取所有投递计划
#[get("/job_define/list")]
pub async fn all_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let job_define_service = JOB_DEFINE_SERVICE.get().unwrap();

    match job_define_service.find_all().await {
        Ok(job_define_res) => {
            let response = ApiResponse::success(job_define_res);
            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            error!("Failed to fetch job defines: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

// TODO 创建投递计划
#[post("/job_define/create")]
pub async fn create_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}

// TODO 更新投递计划
#[put("/job_define/update")]
pub async fn update_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}

// TODO 删除投递计划
#[delete("/job_define/delete")]
pub async fn delete_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}