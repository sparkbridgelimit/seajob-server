use std::env;
use std::future::Future;
use std::pin::Pin;
use actix_web::{Error, error, HttpRequest, HttpResponse, ResponseError};
use actix_web::dev::Payload;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, errors::Error as JwtError};
use once_cell::sync::Lazy;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::auth;
use crate::redis_client::multiplexed_conn;

pub static JWT_SECRET_KEY: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET_KEY").unwrap_or("local-secret-key".parse().unwrap())
});

// 定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

// 定义错误响应格式
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error_code: u16,
    pub error_message: String,
}

// 为错误类型实现 ResponseError，以生成自定义 HttpResponse
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::Unauthorized(msg) => {
                HttpResponse::Ok().json(ErrorResponse {
                    success: false,
                    error_code: 401,
                    error_message: msg.clone(),
                })
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    /// 保存的用户id
    pub user_id: i64,
    pub exp: usize,
}

impl Claims {
    pub fn new(id: i64, exp: usize) -> Self {
        Self {
            user_id: id,
            exp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authenticate {
    pub user_id: i64,
}

pub async fn get_user_from_redis(user_id: i64) -> Option<String> {
    let mut m_conn = multiplexed_conn().await;

    // 使用 Redis GET 命令获取用户数据
    redis::cmd("GET")
        .arg(format!("user:{}", user_id))
        .query_async(&mut m_conn)
        .await
        .unwrap_or_else(|e| {
            eprintln!("无法从 Redis 获取用户数据: {}", e);
            None
        })
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let validation = Validation::new(Algorithm::HS256);
    let key = DecodingKey::from_secret(JWT_SECRET_KEY.as_ref());
    let data = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
    Ok(data)
}

async fn validate_user(user_id: i64) -> Result<Authenticate, Error> {
    // 从 Redis 中获取用户数据的 JSON 字符串
    let user_json = get_user_from_redis(user_id).await
        .ok_or_else(|| error::ErrorUnauthorized("Invalid user in Redis"))?;

    // 尝试将 JSON 字符串反序列化为 UserData 结构体
    let user_data = serde_json::from_str::<Authenticate>(&user_json)
        .map_err(|e| {
            eprintln!("解析 Redis 中的用户数据失败: {}", e);
            error::ErrorInternalServerError("Failed to parse user data")
        })?;

    Ok(user_data)
}

#[async_trait(? Send)]
impl actix_web::FromRequest for Authenticate {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let auth_header = req.headers().get("Authorization")
                .ok_or_else(|| CustomError::Unauthorized("Authorization Not Found".into()))?;

            let token = auth_header
                .to_str()
                .map_err(|_| CustomError::Unauthorized("Invalid Authorization".into()))?
                .strip_prefix("Bearer ")
                .ok_or_else(|| CustomError::Unauthorized("Invalid Authorization".into()))?;

            let data = auth::validate_token(token)
                .map_err(|e| {
                    eprintln!("{}", e);
                    CustomError::Unauthorized("Invalid Authorization".into())
                })?;

            validate_user(data.claims.user_id).await.map_err(|_| CustomError::Unauthorized("Invalid user in Redis".into()))
        })
    }
}