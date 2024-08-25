use crate::{activate, auth};
use actix_web::web;


// 不需要鉴权的部分
pub fn not_auth_routes(cfg: &mut web::ServiceConfig) {
    // 登陆, 注册
    cfg.service(auth::sign_up)
        .service(auth::sign_in);
}

// 需要鉴权的部分
pub fn need_auth_routes(cfg: &mut web::ServiceConfig) {
    // 登出
    cfg.service(auth::sign_out)
        .service(auth::check)
        .service(activate::create)
        .service(activate::consume);
}

// 模块主入口
pub fn entry(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // 不需要认证
            .service(web::scope("/f").configure(not_auth_routes))
            // 需要认证
            .service(web::scope("/s").configure(need_auth_routes)),
    );
}
