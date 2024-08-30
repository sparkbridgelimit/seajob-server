use sea_orm::Set;
use seajob_common::db;
use seajob_entity::member::role;
use sea_orm::{ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};

use crate::err::ServiceError;

pub struct RoleService;


pub struct CreateRoleReq {
    pub name: String,
    pub desc: String,
    pub code: String,
}

impl RoleService {
    /// 创建角色
    pub async fn create(
        req: CreateRoleReq,
    ) -> Result<role::Model, ServiceError> {
        role::ActiveModel {
            id: Default::default(),
            name: Set(req.name),
            desc: Set(req.desc),
            code: Set(req.code),
            create_time: Default::default(),
            update_time: Default::default(),
        }
        .insert(db::conn())
        .await
        .map_err(|e| ServiceError::DbError(e))
    }

    /// 查询角色
    pub async fn query(id: i64) -> Result<Option<role::Model>, ServiceError> {
        role::Entity::find()
            .filter(role::Column::Id.eq(id))
            .one(db::conn())
            .await
            .map_err(|e| ServiceError::DbError(e))
    }
}
