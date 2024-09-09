use crate::err::ServiceError;
use chrono::{DateTime, Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter, TransactionTrait};
use seajob_common::db;
use seajob_common::id_gen::rand_numeric::rand_numeric;
use seajob_dto::req::activate::{ConsumeActivateCodeReq, CreateActivateCodeReq};
use seajob_entity::member::{activation_code, user_activation_log, user_membership};
use seajob_entity::member::activation_code::ActiveModel;
use seajob_entity::member::user_membership::Model;

async fn update_activation_code(
    code: &activation_code::Model,
    txn: &sea_orm::DatabaseTransaction,
) -> Result<ActiveModel, ServiceError> {
    let mut code_active_model: ActiveModel = code.clone().into();
    code_active_model.is_used = Set(true);
    code_active_model
        .save(txn)
        .await
        .map_err(ServiceError::DbError)
}

async fn create_activation_log(
    user_id: i64,
    code: &activation_code::Model,
    expires_at: DateTime<Utc>,
    txn: &sea_orm::DatabaseTransaction,
) -> Result<user_activation_log::Model, ServiceError> {
    user_activation_log::ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        activation_code_id: Set(code.id),
        biz_code: Set(code.biz_code.clone()),
        activated_at: Set(Utc::now()),
        expires_at: Set(expires_at),
        create_time: Default::default(),
        update_time: Default::default(),
    }
        .insert(txn)
        .await
        .map_err(ServiceError::DbError)
}

async fn update_membership(
    membership: &Model,
    new_expires_at: DateTime<Utc>,
    txn: &sea_orm::DatabaseTransaction,
) -> Result<Model, ServiceError> {
    let mut membership_active_model: user_membership::ActiveModel = membership.clone().into();
    membership_active_model.expires_at = Set(new_expires_at);
    membership_active_model.update_time = Set(Some(Utc::now()));
    membership_active_model
        .update(txn)
        .await
        .map_err(ServiceError::DbError)
}

async fn create_membership(
    user_id: i64,
    biz_code: String,
    expires_at: DateTime<Utc>,
    txn: &sea_orm::DatabaseTransaction,
) -> Result<Model, ServiceError> {
    user_membership::ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        biz_code: Set(biz_code),
        expires_at: Set(expires_at),
        create_time: Default::default(),
        update_time: Default::default(),
    }
        .insert(txn)
        .await
        .map_err(ServiceError::DbError)
}

fn calculate_new_expiry(
    membership: &Model,
    duration_days: i64,
) -> DateTime<Utc> {
    let now = Utc::now();
    if membership.expires_at < now {
        // 会员已过期，从当前时间开始增加
        now + Duration::days(duration_days)
    } else {
        // 会员未过期，从当前的失效时间增加
        membership.expires_at + Duration::days(duration_days)
    }
}

pub struct ActivateService;

impl ActivateService {
    pub async fn create(
        req: CreateActivateCodeReq,
    ) -> Result<activation_code::Model, ServiceError> {
        ActiveModel {
            id: Default::default(),
            code: Set(rand_numeric(8)),
            expire_at: Set(Utc::now() + Duration::days(30)),
            biz_code: Set(req.biz_code),
            duration: Set(req.duration),
            is_used: Set(false),
            create_time: Default::default(),
            update_time: Default::default(),
        }
            .insert(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))
    }

    pub async fn consume(
        user_id: i64,
        req: ConsumeActivateCodeReq,
    ) -> Result<bool, ServiceError> {
        // 查询code = req.code and is_used == false
        let code = activation_code::Entity::find()
            .filter(activation_code::Column::Code.eq(req.code))
            .filter(activation_code::Column::IsUsed.eq(false))
            .one(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))?;

        let code = match code {
            Some(c) => c,
            None => {
                return Err(ServiceError::NotFoundError(
                    "激活码不存在或已使用".to_string(),
                ));
            }
        };

        // 查询用户的会员资格记录
        let existed_user_membership = user_membership::Entity::find()
            .filter(user_membership::Column::UserId.eq(user_id))
            .filter(user_membership::Column::BizCode.eq("seajob|user|try".to_string()))
            .one(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))?;

        let now = Utc::now();
        // 获取激活码有效期
        let duration_days = code.duration;

        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    // 更新激活码状态
                    update_activation_code(&code, txn).await?;

                    // 会员的失效时间，基于当前时间或会员的现有到期时间
                    let new_expires_at = match existed_user_membership {
                        Some(ref membership) => calculate_new_expiry(membership, duration_days),
                        None => now + Duration::days(duration_days),
                    };

                    // 创建激活记录
                    create_activation_log(user_id, &code, new_expires_at, txn).await?;

                    // 更新或创建会员资格
                    if let Some(membership) = existed_user_membership {
                        update_membership(&membership, new_expires_at, txn).await?;
                    } else {
                        create_membership(user_id, code.biz_code, new_expires_at, txn).await?;
                    }

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))
    }
}
