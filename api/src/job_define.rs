use actix_web::{delete, Error, get, HttpRequest, HttpResponse, post, put, web};
use seajob_common::response::{ApiErr, ApiResponse};

use crate::AppState;
use seajob_service::entry::JOB_DEFINE_SERVICE;
use log::error;
use seajob_dto::req::job_define_create::JobDefineCreateRequest;
use seajob_service::job_define::JobDefineService;

// TODO: 获取所有投递计划
#[get("/list")]
pub async fn all_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let job_define_service = JOB_DEFINE_SERVICE.get().unwrap();

    match job_define_service.find_all().await {
        Ok(job_define_res) => {
            let response = ApiResponse::success(job_define_res);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to fetch job defines: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

// TODO 创建投递计划
#[post("/create")]
pub async fn create_job_define(json: web::Json<JobDefineCreateRequest>) -> Result<HttpResponse, Error> {
    let req = json.into_inner();
    match JobDefineService::create(req).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
        }
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// TODO 更新投递计划
#[put("/update")]
pub async fn update_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}

// TODO 删除投递计划
#[delete("/delete")]
pub async fn delete_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");
    Ok(HttpResponse::Ok().json(response))
}