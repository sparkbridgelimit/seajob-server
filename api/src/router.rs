use actix_web::web;
use crate::{index, job_contacted, job_define};

fn index_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index::index);
}

// 已投工作模块
fn job_contacted_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(job_contacted::all_contacted_job)
        .service(job_contacted::all_contacted_by_job_define)
        .service(job_contacted::contacted_by_job_task_all)
        .service(job_contacted::contacted_by_job_task_create);
}

// 投递计划模块
fn job_define_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(job_define::all_job_define)
        .service(job_define::create_job_define)
        .service(job_define::update_job_define)
        .service(job_define::delete_job_define);
}

// 模块主入口
pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/").configure(index_routes))
        .service(web::scope("/job_define").configure(job_define_routes))
        .service(web::scope("/job_contacted").configure(job_contacted_routes));
}