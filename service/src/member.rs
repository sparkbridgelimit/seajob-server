use crate::err::ServiceError;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use seajob_common::db;
use seajob_entity::member::user_membership;

pub struct MemberShipService;

impl MemberShipService {
    /// 查询用户
    pub async fn query(
        user_id: i64,
    ) -> Result<user_membership::Model, ServiceError> {
        // 查询用户是否有membership记录
        user_membership::Entity::find()
            .filter(user_membership::Column::UserId.eq(user_id))
            .one(db::conn())
            .await
            .map_err(ServiceError::DbError)?
            .ok_or(ServiceError::NotFoundError(
                "membership not found".to_string(),
            ))
    }

    // pub async fn check(
    //     user_id: i64,
    // ) -> Result<user_membership::Model, ServiceError> {
    //     todo!()
    // }

    // pub async fn join(
    //     req: CreateActivateCodeReq,
    // ) -> Result<activation_code::Model, ServiceError> {
    //     todo!()
    // }
}
