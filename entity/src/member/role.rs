/// RBAC 角色
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use serde::Serialize;

use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key, comment = "Primary key")]
    pub id: i64,

    #[sea_orm(comment = "角色名")]
    pub name: String,

    #[sea_orm(comment = "描述")]
    pub desc: String,

    #[sea_orm(comment = "角色编码")]
    pub code: String,

    #[sea_orm(comment = "创建时间")]
    pub create_time: Option<DateTime<Utc>>,

    #[sea_orm(comment = "更新时间")]
    pub update_time: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };
        Self {
            id: Set(id),
            create_time: Set(Option::from(Utc::now())),
            update_time: Set(Option::from(Utc::now())),
            ..ActiveModelTrait::default()
        }
    }

    // 插入或者更新前触发
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();
        let id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };

        // 如果没有设置id, 则默认给一个
        if self.id.is_not_set() {
            self.id = Set(id);
        }

        // 新插入的则设置创建时间
        if insert {
            self.create_time = Set(Option::from(now));
        }

        self.update_time = Set(Option::from(now));

        Ok(self)
    }
}
