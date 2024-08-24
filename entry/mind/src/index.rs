use actix_web::{get, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

// 定义通用的ApiResponse结构体
#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error_code: Option<u32>,
    error_message: Option<String>,
}

// 实现 ApiResponse 的构造函数
impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error_code: None,
            error_message: None,
        }
    }
}

// 定义 ApiResponder 类型来处理泛型返回
pub struct ApiResponder<T>(pub T);

// 为泛型 T 实现 Responder trait，使其自动包装到 ApiResponse 中
// 为 ApiResponder<T> 实现 Responder trait
impl<T> Responder for ApiResponder<T>
where
    T: Serialize,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let api_response = ApiResponse::success(self.0);
        HttpResponseBuilder::new(StatusCode::OK).json(api_response)
    }
}

// 为 Responder 提供泛型实现，使其自动转换为 ApiResponder
impl<T> From<T> for ApiResponder<T> {
    fn from(data: T) -> Self {
        ApiResponder(data)
    }
}

struct NewResponder<T>(T);

// 为 NewResponder 实现 Responder，间接实现对泛型 T 的支持
impl<T> Responder for NewResponder<T>
where
    T: Serialize,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        ApiResponder::from(self.0).respond_to(req)
    }
}

#[derive(Serialize)]
struct Age {
    value: i64,

}

#[derive(Serialize)]
struct User {
    name: String,
    age: Age,
}


#[get("/")]
async fn index() -> NewResponder<User> {
    NewResponder(User {
        name: "asd".to_string(),
        age: Age {
            value: 12
        },
    })
}
