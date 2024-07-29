use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use seajob_common::response::ApiResponse;
use crate::AppState;

// TODO: 查看用户所有已沟通的岗位, 不区分计划和任务
#[get("/by/user/all")]
pub async fn all_contacted_job(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("/contacted/by/user/all");
    Ok(HttpResponse::Ok().json(response))
}

// TODO: 查看用户某一个计划的所有已沟通的岗位
#[get("/by/job_define/list")]
pub async fn all_contacted_by_job_define(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("/contacted/by/job_define/list");
    Ok(HttpResponse::Ok().json(response))
}

// TODO: 查看用户某一个已执行任务的所有已经沟通岗位
#[get("/by/job_task/list")]
pub async fn contacted_by_job_task_all(_req: HttpRequest, _: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ApiResponse::success("/contacted/by/job_task/list");
    Ok(HttpResponse::Ok().json(response))
}
