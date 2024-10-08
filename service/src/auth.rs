use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use redis::AsyncCommands;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

use seajob_common::auth::{Claims, JWT_SECRET_KEY};
use seajob_common::db;
use seajob_common::err::ServiceError;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;
use seajob_common::redis_client::multiplexed_conn;
use seajob_dto::req::auth::{SignInPayload, SignUpRequest};
use seajob_dto::res::auth::{SignInResponse, SignUpResponse};
use seajob_entity::{account, member::user_role, user_define};
use seajob_entity::member::role;

/// 创建token
pub fn create_jwt(id: i64) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let header = Header::new(Algorithm::HS256);
    let claims = Claims::new(id, expiration as usize);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
        .map(|s| format!("Bearer {}", s))
        .unwrap()
}

#[derive(FromQueryResult)]
struct LimitedAccount {}

// DONE: 注册
pub async fn sign_up(params: SignUpRequest) -> Result<SignUpResponse, ServiceError> {
    let user_id = {
        let id_gen = GLOBAL_IDGEN.lock().unwrap();
        id_gen.next_id().unwrap()
    };

    db::conn()
        .transaction::<_, _, ServiceError>(|txn| {
            Box::pin(async move {
                // 查看用户需要注册的用户名是否已经存在了
                let existing_account = account::Entity::find()
                    .select_only()
                    .column(account::Column::Id)
                    .filter(account::Column::ProviderAccountId.eq(params.username.clone()))
                    .into_model::<LimitedAccount>()
                    .one(txn)
                    .await?;

                // 如果重复直接返回ServiceError
                if existing_account.is_some() {
                    return Err(ServiceError::ConflictError(
                        "Username already exists".into(),
                    ));
                }

                // 如果 accounts 中没有记录, 创建user_define、account记录
                user_define::ActiveModel {
                    id: Set(user_id),
                    status: Set(String::from("active")),
                    extra: Default::default(),
                    create_time: Default::default(),
                    update_time: Default::default(),
                }
                    .insert(txn)
                    .await?;

                // 对密码进行哈希处理
                let hashed_password = hash(&params.password, DEFAULT_COST)
                    .map_err(|e| ServiceError::BizError(e.to_string()))?;

                // 在 accounts 表中创建新的认证方式记录
                account::ActiveModel {
                    id: Default::default(),
                    user_id: Set(user_id),
                    provider_type: Set("credentials".to_string()),
                    provider_id: Set("password".to_string()),
                    provider_account_id: Set(params.username.clone()),
                    access_token: Set(hashed_password),
                    create_time: Set(Some(Utc::now())),
                    update_time: Set(Some(Utc::now())),
                    ..Default::default()
                }
                    .insert(txn)
                    .await?;

                let code = "user";
                let role = role::Entity::find()
                    .filter(role::Column::Code.eq(code))
                    .one(txn)
                    .await?
                    .ok_or_else(|| ServiceError::NotFoundError("Role not found".into()))?;

                user_role::ActiveModel {
                    id: Default::default(),
                    user_id: Set(user_id),
                    role_id: Set(role.id),
                    role_code: Set(role.code),
                    create_time: Default::default(),
                    update_time: Default::default(),
                }
                    .insert(txn)
                    .await?;
                Ok(true)
            })
        })
        .await
        .map_err(|e| ServiceError::TransactionError(Box::new(e)))?;

    // 生成 JWT Token
    let claims = Claims {
        user_id,
        exp: Utc::now().timestamp() as usize + 3600 * 24, // Token 1小时过期
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
        .map_err(|e| ServiceError::BizError(e.to_string()))?;

    let mut redis_conn = multiplexed_conn().await;

    let mut pipeline = redis::pipe();

    let expire_time = 3600 * 24;

    pipeline.set_ex(
        format!("user:{}:token", user_id),
        token.clone(),
        expire_time,
    ).ignore();

    let user_roles = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .all(db::conn())
        .await?;

    for user_role in user_roles {
        pipeline.sadd(
            format!("user:{}:roles", user_role.user_id),
            user_role.role_code,
        ).ignore();
    }

    pipeline
        .query_async(&mut redis_conn)
        .await
        .map_err(|e| ServiceError::SystemError(e.to_string()))?;

    // 返回token
    Ok(SignUpResponse {
        token,
        exp_at: claims.exp,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedUserData {
    user_id: i64,
}

pub async fn sign_in(params: SignInPayload) -> Result<SignInResponse, ServiceError> {
    // 获取用户名对应的账户记录
    let account = account::Entity::find()
        .filter(account::Column::ProviderAccountId.eq(params.username.clone()))
        .filter(account::Column::ProviderType.eq("credentials"))
        .one(db::conn())
        .await?
        .ok_or(ServiceError::BizError(
            "Invalid username or password".into(),
        ))?;

    // 比对密码是否正确
    let is_valid = verify(&params.password, &account.access_token)
        .map_err(|_| ServiceError::BizError("Password verification failed".into()))?;

    if !is_valid {
        return Err(ServiceError::BizError(
            "Invalid username or password".into(),
        ));
    }

    // 生成 JWT Token
    let claims = Claims {
        user_id: account.user_id,
        exp: Utc::now().timestamp() as usize + 3600 * 24, // Token 1小时过期
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
        .map_err(|e| ServiceError::BizError(e.to_string()))?;

    let mut redis_conn = multiplexed_conn().await;

    let mut pipeline = redis::pipe();

    let expire_time = 3600 * 24;

    pipeline.set_ex(
        format!("user:{}:token", account.user_id),
        token.clone(),
        expire_time,
    ).ignore();

    let user_roles = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(account.user_id))
        .all(db::conn())
        .await?;

    for user_role in user_roles {
        pipeline.sadd(
            format!("user:{}:roles", user_role.user_id),
            user_role.role_code,
        ).ignore();
    }

    pipeline.expire(format!("user:{}:roles", account.user_id), expire_time as i64).ignore();

    pipeline
        .query_async(&mut redis_conn)
        .await
        .map_err(|e| ServiceError::SystemError(e.to_string()))?;

    // 返回token
    Ok(SignInResponse {
        token,
        exp_at: claims.exp,
    })
}

pub async fn sign_out(user_id: i64) -> Result<bool, ServiceError> {
    let mut redis_conn = multiplexed_conn().await;

    // 从 Redis 中删除用户的 token
    let result: i64 = redis_conn
        .del(format!("user:{}:token", user_id))
        .await
        .map_err(|e| ServiceError::SystemError(e.to_string()))?;

    if result == 1 {
        Ok(true)
    } else {
        Err(ServiceError::SystemError(
            "Sign out failed: token not found".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let jwt_token = create_jwt(123456);
        println!("{:}", jwt_token);
    }
}
