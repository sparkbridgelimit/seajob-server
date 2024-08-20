use std::future::Future;
use std::pin::Pin;
use actix_web::{get, Error, HttpRequest, HttpResponse, error, post, web};
use actix_web::dev::Payload;
use async_trait::async_trait;
use log::error;
use serde::{Deserialize, Serialize};
use seajob_common::response::ApiResponse;
use seajob_dto::req::auth::{SignInPayload, SignUpRequest};
use seajob_service::auth;
use seajob_service::auth::get_user_from_redis;

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    user_id: i64,
}

async fn validate_user(user_id: i64) -> Result<UserData, Error> {
    // 从 Redis 中获取用户数据的 JSON 字符串
    let user_json = get_user_from_redis(user_id).await
        .ok_or_else(|| error::ErrorUnauthorized("Invalid user in Redis"))?;

    // 尝试将 JSON 字符串反序列化为 UserData 结构体
    let user_data = serde_json::from_str::<UserData>(&user_json)
        .map_err(|e| {
            eprintln!("解析 Redis 中的用户数据失败: {}", e);
            error::ErrorInternalServerError("Failed to parse user data")
        })?;

    Ok(user_data)
}

#[async_trait(? Send)]
impl actix_web::FromRequest for UserData {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let auth_header = req.headers().get("Authorization")
                .ok_or_else(|| error::ErrorUnauthorized("Authorization Not Found"))?;

            let token = auth_header
                .to_str()
                .map_err(|_| error::ErrorBadRequest("Invalid Authorization"))?
                .strip_prefix("Bearer ")
                .ok_or_else(|| error::ErrorBadRequest("Invalid Authorization"))?;

            let data = auth::validate_token(token)
                .map_err(|e| {
                    eprintln!("{}", e);
                    error::ErrorBadRequest("Invalid Authorization")
                })?;

            validate_user(data.claims.user_id).await
        })
    }
}

// TODO: traefik中间件鉴权验证
#[get("/check")]
async fn check(_user: UserData) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("x-user-id", _user.user_id))
        .json(ApiResponse::success_only()))
}

#[post("/sign_up")]
async fn sign_up(json: web::Json<SignUpRequest>) -> Result<HttpResponse, Error> {
    match auth::sign_up(json.into_inner()).await {
        Ok(_) => {
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign up: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/sign_in")]
async fn sign_in(json: web::Json<SignInPayload>) -> Result<HttpResponse, Error> {
    match auth::sign_in(json.into_inner()).await {
        Ok(_) => {
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign in: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}

#[post("/sign_out")]
async fn sign_out(user: UserData) -> Result<HttpResponse, Error> {
        match auth::sign_out(user.user_id).await {
        Ok(_) => {
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to sign in: {:?}", e);
            let error_response = ApiResponse::fail();
            Ok(HttpResponse::Ok().json(error_response))
        }
    }
}