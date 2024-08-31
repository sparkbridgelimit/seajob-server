use crate::redis_client::multiplexed_conn;
use actix_web::dev::Payload;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use jsonwebtoken::{errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use redis::AsyncCommands;

pub static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("JWT_SECRET_KEY").unwrap_or("local-secret-key".parse().unwrap()));

// 定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
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
            CustomError::Unauthorized(msg) => HttpResponse::Ok().json(ErrorResponse {
                success: false,
                error_code: 401,
                error_message: msg.clone(),
            }),
            CustomError::SystemError(msg) => HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error_code: 500,
                error_message: msg.clone(),
            }),
            CustomError::RedisError(err) => HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error_code: 500,
                error_message: err.to_string(),
            }),
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
        Self { user_id: id, exp }
    }
}

// 定义角色的 Trait
pub trait Role: Send + Sync + 'static {
    fn role_name() -> &'static str;
}

pub struct AdminRole;
pub struct UserRole;

impl Role for AdminRole {
    fn role_name() -> &'static str {
        "admin"
    }
}

impl Role for UserRole {
    fn role_name() -> &'static str {
        "user"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authenticate<R: RoleList> {
    pub user_id: i64,
    pub role_codes: Vec<String>, // 支持多个角色
    _marker: PhantomData<R>,
}

// RoleList Trait 用于表示角色列表
pub trait RoleList: Send + Sync + 'static {
    fn required_roles() -> Vec<&'static str>;
    fn has_role(user_roles: &[String]) -> bool;
}

// 为单个角色实现 RoleList
impl<T: Role> RoleList for T {
    fn required_roles() -> Vec<&'static str> {
        vec![T::role_name()]
    }

    fn has_role(user_roles: &[String]) -> bool {
        user_roles.contains(&T::role_name().to_string())
    }
}

// 为两个角色组合实现 RoleList
impl<T1: Role, T2: Role> RoleList for (T1, T2) {
    fn required_roles() -> Vec<&'static str> {
        vec![T1::role_name(), T2::role_name()]
    }

    fn has_role(user_roles: &[String]) -> bool {
        user_roles.contains(&T1::role_name().to_string()) || user_roles.contains(&T2::role_name().to_string())
    }
}


pub async fn get_user_from_redis(user_id: i64) -> Option<String> {
    let mut m_conn = multiplexed_conn().await;

    // 使用 Redis GET 命令获取用户数据
    redis::cmd("GET")
        .arg(format!("user:{}:token", user_id))
        .query_async(&mut m_conn)  // 使用可变引用
        .await
        .unwrap_or_else(|e| {
            eprintln!("无法从 Redis 获取用户数据: {}", e);
            None
        })
}

async fn get_user_roles(user_id: i64,) -> Result<Vec<String>, CustomError> {
    let mut redis_conn = multiplexed_conn().await; // 获取一个可变引用

    let roles: Vec<String> = redis_conn
        .smembers(format!("user:{}:roles", user_id))
        .await
        .map_err(CustomError::from)?;

    Ok(roles)
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let validation = Validation::new(Algorithm::HS256);
    let key = DecodingKey::from_secret(JWT_SECRET_KEY.as_ref());
    let data = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
    Ok(data)
}

async fn validate_user<R: RoleList>(user_id: i64, token: &str) -> Result<Authenticate<R>, CustomError> {
    // 从 Redis 中获取用户数据的 JSON 字符串
    let user_token = get_user_from_redis(user_id)
        .await
        .ok_or_else(|| CustomError::Unauthorized("Failed to get user from Redis".into()))?;

    // 判断redis存的token和用户的token是否一致
    if user_token != token {
        return Err(CustomError::Unauthorized("无效的token".into()));
    }

    let required_roles = R::required_roles();
    let user_roles = get_user_roles(user_id)
        .await
        .map_err(|e| CustomError::Unauthorized(e.to_string()))?;

    // 检查用户是否具有所有必需的角色
    for role in required_roles {
        if !user_roles.contains(&role.to_string()) {
            return Err(CustomError::Unauthorized("没有相关权限".into()));
        }
    }

    // 校验用户是否有权限
    if R::has_role(&user_roles) {
        Ok(Authenticate {
            user_id,
            role_codes: user_roles,
            _marker: PhantomData::<R>,
        })
    } else {
        Err(CustomError::Unauthorized("没有相关权限".into()))
    }
}

#[async_trait(? Send)]
impl<R: RoleList> actix_web::FromRequest for Authenticate<R> {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            // 提取 Authorization 头部
            let auth_header = req
                .headers()
                .get("Authorization")
                .ok_or_else(|| CustomError::Unauthorized("Authorization Not Found".into()))?;

            // 提取 token
            let token = auth_header
                .to_str()
                .map_err(|_| CustomError::Unauthorized("Invalid Authorization".into()))?
                .strip_prefix("Bearer ")
                .ok_or_else(|| CustomError::Unauthorized("Invalid Authorization".into()))?;

            // 校验token
            let data = validate_token(token).map_err(|e| {
                eprintln!("{}", e);
                CustomError::Unauthorized("Invalid Authorization".into())
            })?;

            // 校验用户是否在 Redis 中并检查权限
            validate_user::<R>(data.claims.user_id, token).await
        })
    }
}
