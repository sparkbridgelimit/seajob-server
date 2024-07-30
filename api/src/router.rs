use crate::{index, job_contacted, job_define, job_task};
use actix_web::web;

fn index_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
}

// 已投工作模块
fn job_contacted_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(job_contacted::all_contacted_job)
        .service(job_contacted::all_contacted_by_job_define)
        .service(job_contacted::contacted_by_job_task_all);
}

// 投递计划模块
fn job_define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(job_define::all_job_define)
        .service(job_define::create_job_define)
        .service(job_define::update_job_define)
        .service(job_define::delete_job_define)
        .service(job_define::run);
}

fn job_task(cfg: &mut web::ServiceConfig) {
    cfg.service(job_task::list)
        .service(job_task::start)
        .service(job_task::log_task)
        .service(job_task::error)
        .service(job_task::end);
}

// 模块主入口
pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/").configure(index_routes))
            .service(web::scope("/job_define").configure(job_define_routes))
            .service(web::scope("/job_contacted").configure(job_contacted_routes))
            .service(web::scope("/job_task").configure(job_task))
    );
}
