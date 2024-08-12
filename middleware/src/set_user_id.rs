use actix_web::{HttpRequest, web};

async fn set_user_middleware(req: HttpRequest, next: web::ServiceConfig) -> impl Responder {
    // 在 req.extensions() 中存储数据
    req.extensions_mut().insert(MyData {
        user_id: "12345".to_string(),
    });

    // 继续处理请求
    next.call(req).await
}