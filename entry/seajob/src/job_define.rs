use actix_web::{post, web, Error, HttpResponse};
use log::error;
use seajob_common::auth::Authenticate;
use seajob_common::response::{ApiErr, ApiResponse};
use seajob_dto::req::job_define::{JobDefineCookieRequest, JobDefineCreateRequest, JobDefineDelete, JobDefineDetailRequest, JobDefineRunRequest, JobDefineSaveCookieRequest, JobDefineUpdateRequest};
use seajob_service::job_define::JobDefineService;

// DONE: 获取用户的所有投递计划
// #[get("/user/{user_id}")]
#[post("/list")]
pub async fn all_job_define(user_context: Authenticate) -> Result<HttpResponse, Error> {
    match JobDefineService::find_all_by_user(user_context.user_id).await {
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
    user_context: Authenticate,
) -> Result<HttpResponse, Error> {
    let params = json.into_inner();
    match JobDefineService::create(params, user_context.user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(true))),
        Err(e) => {
            error!("Failed to create job defines: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

#[post("/detail")]
pub async fn query_detail(
    json: web::Json<JobDefineDetailRequest>,
    user_context: Authenticate,
) -> Result<HttpResponse, Error> {
    let params = json.into_inner();
    match JobDefineService::detail(params, user_context.user_id).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(res))),
        Err(e) => {
            error!("Failed to query detail of job define: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

#[post("/get_cookie")]
pub async fn get_cookie(
    json: web::Json<JobDefineCookieRequest>,
    _: Authenticate,
) -> Result<HttpResponse, Error> {
    let params = json.into_inner();
    match JobDefineService::get_cookie(params).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(res))),
        Err(e) => {
            error!("Failed to get_cookie: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

#[post("/update")]
pub async fn update_job_define(
    json: web::Json<JobDefineUpdateRequest>,
) -> Result<HttpResponse, Error> {
    let params = json.into_inner();
    match JobDefineService::update(params).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(true))),
        Err(e) => {
            error!("Failed to update job defines: {:?}", e);
            let error_response = ApiResponse::fail_with_error(ApiErr::SYSTEM);
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}

#[post("/delete")]
pub async fn delete_job_define(
    req: web::Json<JobDefineDelete>,
) -> Result<HttpResponse, Error> {
    match JobDefineService::delete(req.into_inner()).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to delete job defines: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/run")]
pub async fn run(user_context: Authenticate, json: web::Json<JobDefineRunRequest>) -> Result<HttpResponse, Error> {
    let params = json.into_inner();
    match JobDefineService::run(params, user_context.user_id).await {
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

#[post("/cookie")]
pub async fn save_cookie(json: web::Json<JobDefineSaveCookieRequest>) -> Result<HttpResponse, Error> {
    match JobDefineService::save_cookie(json.into_inner()).await {
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
