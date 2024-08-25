use crate::err::ServiceError;
use chrono::{Duration, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter, TransactionTrait};
use seajob_common::db;
use seajob_common::id_gen::rand_numeric::rand_numeric;
use seajob_dto::req::activate::{ConsumeActivateCodeReq, CreateActivateCodeReq};
use seajob_entity::member::{activation_code, user_activation_log, user_membership};

pub struct ActivateService;

impl ActivateService {
    pub async fn create(
        req: CreateActivateCodeReq,
    ) -> Result<activation_code::Model, ServiceError> {
        activation_code::ActiveModel {
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

        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    // 更新is_used = true
                    let mut code_active_model: activation_code::ActiveModel = code.into();
                    code_active_model.is_used = Set(true);

                    // 提前提取所需的字段值
                    let duration_value = code_active_model.duration.clone().unwrap();
                    let biz_code_value = code_active_model.biz_code.clone().unwrap();
                    let activation_code_id_value = code_active_model.id.clone().unwrap();

                    code_active_model.save(txn).await?;

                    // 会员生效时间
                    let now = Utc::now();
                    // 会员失效时间
                    let expires_at = now + Duration::days(duration_value);

                    // 创建激活记录
                    user_activation_log::ActiveModel {
                        id: Default::default(),
                        user_id: Set(user_id),
                        activation_code_id: Set(activation_code_id_value),
                        biz_code: Set(biz_code_value.clone()), // 使用解包后的值
                        activated_at: Set(now),
                        expires_at: Set(expires_at),
                        create_time: Default::default(),
                        update_time: Default::default(),
                    }
                        .insert(txn)
                        .await
                        .map_err(|e| ServiceError::DbError(e))?;

                    // 创建会员资格记录
                    user_membership::ActiveModel {
                        id: Default::default(),
                        user_id: Set(user_id),
                        biz_code: Set(biz_code_value), // 使用解包后的值
                        expires_at: Set(expires_at),
                        create_time: Default::default(),
                        update_time: Default::default(),
                    }
                        .insert(txn)
                        .await
                        .map_err(|e| ServiceError::DbError(e))?;

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))
    }
}
