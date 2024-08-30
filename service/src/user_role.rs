use sea_orm::Set;
use seajob_common::db;
use seajob_entity::member::{role, user_role};
use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};

use crate::err::ServiceError;

pub struct UserRoleService;

pub struct CreateUserRoleReq {
    pub user_id: i64,
    pub role_id: i64,
    pub role_code: String,
    pub name: String,
    pub desc: String,
}

impl UserRoleService {
    /// 创建角色
    pub async fn create(
        req: CreateUserRoleReq,
    ) -> Result<user_role::Model, ServiceError> {
        user_role::ActiveModel {
            id: Default::default(),
            user_id: Set(req.user_id),
            role_id: Set(req.role_id),
            role_code: Set(req.role_code),
            create_time: Default::default(),
            update_time: Default::default(),
        }
        .insert(db::conn())
        .await
        .map_err(|e| ServiceError::DbError(e))
    }

    /// 查询角色详情
    pub async fn query(id: i64) -> Result<Option<user_role::Model>, ServiceError> {
        user_role::Entity::find()
            .filter(role::Column::Id.eq(id))
            .one(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))
    }

    /// 查询用户角色
    pub async fn query_user_roles(user_id: i64) -> Result<Vec<user_role::Model>, ServiceError> {
        user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))
    }
}
