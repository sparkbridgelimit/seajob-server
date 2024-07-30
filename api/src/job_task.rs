use actix_web::{Error, HttpResponse, post, web};
use log::error;
use seajob_common::response::ApiResponse;
use seajob_dto::req::job_task::{JobTaskEnd, JobTaskError, JobTaskList, JobTaskLog, JobTaskStart};
use seajob_service::job_task::JobTaskService;

#[post("/list")]
pub async fn list(req: web::Json<JobTaskList>) -> Result<HttpResponse, Error> {
    match JobTaskService::list(req.into_inner()).await {
        Ok(list) => {
            let response = ApiResponse::success(list);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to list job tasks: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/start")]
pub async fn start(req: web::Json<JobTaskStart>) -> Result<HttpResponse, Error> {
    match JobTaskService::start(req.into_inner()).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
        }
        Err(e) => {
            error!("Failed to log job task: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/log")]
pub async fn log_task(req: web::Json<JobTaskLog>) -> Result<HttpResponse, Error> {
    match JobTaskService::log(req.into_inner()).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
        }
        Err(e) => {
            error!("Failed to log job task: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/error")]
pub async fn error(req: web::Json<JobTaskError>) -> Result<HttpResponse, Error> {
    match JobTaskService::error(req.into_inner()).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
        }
        Err(e) => {
            error!("Failed to log job task: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/end")]
pub async fn end(req: web::Json<JobTaskEnd>) -> Result<HttpResponse, Error> {
    match JobTaskService::end(req.into_inner()).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
        }
        Err(e) => {
            error!("Failed to log job task: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}
