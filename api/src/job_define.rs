use actix_web::{get, post, put, web, Error, HttpRequest, HttpResponse};
use log::error;

use seajob_common::response::{ApiErr, ApiResponse};
use seajob_dto::req::job_define::{JobDefineCreateRequest, JobDefineDelete, JobDefineDetailRequest, JobDefineRunRequest};
use seajob_service::job_define::JobDefineService;

use crate::AppState;

// DONE: 获取用户的所有投递计划
#[get("/user/{user_id}")]
pub async fn all_job_define(user_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let user_id: i64 = match user_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            error!("Validation error: user_id must be a valid i64");
            return Ok(
                HttpResponse::Ok().json(ApiResponse::fail_with_error(ApiErr::ValidationErrors))
            );
        }
    };
    match JobDefineService::find_all_by_user(user_id).await {
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

// DONE 创建投递计划
#[post("/create")]
pub async fn create_job_define(
    json: web::Json<JobDefineCreateRequest>,
) -> Result<HttpResponse, Error> {
    let req = json.into_inner();
    match JobDefineService::create(req).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(true))),
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

#[post("/detail")]
pub async fn query_detail(req: web::Json<JobDefineDetailRequest>) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    match JobDefineService::detail(req).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(res))),
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

// TODO 更新投递计划
#[put("/")]
pub async fn update_job_define(
    _req: HttpRequest,
    _: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("hello man");

    Ok(HttpResponse::Ok().json(response))
}

// TODO 删除投递计划
#[post("/delete")]
pub async fn delete_job_define(
    req: web::Json<JobDefineDelete>,
    _: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    match JobDefineService::delete(req.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/run")]
pub async fn run(req: web::Json<JobDefineRunRequest>) -> Result<HttpResponse, Error> {
    match JobDefineService::run(req.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}
