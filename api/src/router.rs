use actix_web::web;
use crate::{index, job_define};

// 路由配置
pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(index::index)
        .service(index::find_contacted_job)
        .service(job_define::all_job_define);
}