use actix_web::{get, web, Error, HttpResponse};
use log::error;

use seajob_common::response::ApiResponse;
use seajob_dto::req::job_contacted::{JobContactedDefine, JobContactedTaskReq, JobContactedUser};
use seajob_service::job_contacted::JobContactedService;

// Done: 查看用户所有已沟通的岗位, 不区分计划和任务
#[get("/user/all")]
pub async fn all_contacted_job(req: web::Json<JobContactedUser>) -> Result<HttpResponse, Error> {
    match JobContactedService::find_all_by_user(req.into_inner()).await {
        Ok(list) => Ok(HttpResponse::Ok().json(ApiResponse::success(list))),
        Err(e) => {
            error!("job contacted not found: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

// DONE: 查看用户某一个计划的所有已沟通的岗位
#[get("/job_define/all")]
pub async fn all_contacted_by_job_define(
    req: web::Json<JobContactedDefine>,
) -> Result<HttpResponse, Error> {
    match JobContactedService::find_all_by_job_define(req.into_inner()).await {
        Ok(list) => Ok(HttpResponse::Ok().json(ApiResponse::success(list))),
        Err(e) => {
            error!("job contacted not found: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

// TODO: 查看用户某一个已执行任务的所有已经沟通岗位
#[get("/job_task/all")]
pub async fn contacted_by_job_task_all(
    req: web::Json<JobContactedTaskReq>,
) -> Result<HttpResponse, Error> {
    match JobContactedService::find_all_by_job_task(req.into_inner()).await {
        Ok(list) => Ok(HttpResponse::Ok().json(ApiResponse::success(list))),
        Err(e) => {
            error!("job contacted not found: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}
