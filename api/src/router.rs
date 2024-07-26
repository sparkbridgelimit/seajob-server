use actix_web::web;
use crate::index;

// 路由配置
pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(index::index)
        .service(index::find_contacted_job);
}