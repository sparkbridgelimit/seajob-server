use actix_web::web;
use crate::index;

fn index_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
}


// 不需要鉴权的部分
pub fn not_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/index").configure(index_routes));
}

// 需要鉴权的部分
pub fn need_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/index").configure(index_routes));
}

// 模块主入口
pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mind/api")
            .service(web::scope("/f").configure(not_auth_routes))
            .service(web::scope("/s").configure(need_auth_routes)),
    );
}
