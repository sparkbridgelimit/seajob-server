use actix_web::{post, Error, HttpResponse};
use log::error;
use sea_orm::sqlx::types::chrono::Utc;
use seajob_common::auth::{Authenticate, UserRole};
use seajob_common::response::ApiResponse;
use seajob_service::err::ServiceError;
use seajob_service::member::MemberShipService;

/// 查询用户的会员资格
#[post("/member/info")]
pub async fn info(user: Authenticate<UserRole>) -> Result<HttpResponse, Error> {
    // 查询用户是否有membership记录
    match MemberShipService::query(user.user_id).await {
        Ok(data) => {
            let response = ApiResponse::success(data);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            error!("Failed to get membership info: {:?}", e);
            let response = ApiResponse::success_only();
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

/// 检查用户当前是否有会员资格
/// 用户没有会员资格, 也可能有, 也可能过期了
#[post("/member/check")]
pub async fn check(user: Authenticate<UserRole>) -> Result<HttpResponse, Error> {
    match MemberShipService::query(user.user_id).await {
        Ok(membership) => {
            // 判断会员资格是否在有效期内
            if membership.expires_at > Utc::now() {
                Ok(HttpResponse::Ok().json(ApiResponse::success(true)))
            } else {
                Ok(HttpResponse::Ok().json(ApiResponse::success(false))) // 已过期返回 false
            }
        }
        Err(ServiceError::NotFoundError(_)) => {
            // 没有找到会员资格，返回 false
            Ok(HttpResponse::Ok().json(ApiResponse::success(false)))
        }
        Err(_) => {
            // 如果出现其他错误，返回内部服务器错误
            Ok(HttpResponse::Ok().json(ApiResponse::fail()))
        }
    }
}