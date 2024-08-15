use std::env;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, TokenData, errors::Error as JwtError, Validation, DecodingKey};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use seajob_common::redis_client::multiplexed_conn;

static JWT_SECRET_KEY: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET_KEY").unwrap_or("local-secret-key".parse().unwrap())
});

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

/// 创建token
pub fn create_jwt(id: i64) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let header = Header::new(Algorithm::HS256);
    let claims = Claims::new(id, expiration as usize);
    jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()))
        .map(|s| format!("Bearer {}", s))
        .unwrap()
}

/// 验证token
pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let validation = Validation::new(Algorithm::HS256);
    let key = DecodingKey::from_secret(JWT_SECRET_KEY.as_ref());
    let data = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
    Ok(data)
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

// TODO: 注册
// pub async fn signup () {
//
// }

// TODO: 登陆
// pub async fn login() {
//     // 查找用户是否存在
//     // 校验密码
//     // 生成token
//     // 放进redis中
//     // 返回结构体
// }

// TODO: 登出
// pub async fn logout() {
//     // redis中删除对应的key
// }

// TODO: check
// pub async fn check () {
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let jwt_token = create_jwt(123456);
        println!("{:}", jwt_token);
    }
}
