use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::async_trait::async_trait;
use serde::Serialize;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "job_define")]
pub struct Model {
    #[sea_orm(primary_key, comment = "Primary key")]
    pub id: i64,

    #[sea_orm(comment = "关联的用户id")]
    pub user_id: i64,

    #[sea_orm(comment = "投递计划名")]
    pub job_define_name: String,

    #[sea_orm(comment = "投递计划描述")]
    pub job_define_desc: String,

    #[sea_orm(comment = "单次沟通数")]
    pub greet_num: i32,

    #[sea_orm(comment = "创建时间")]
    pub create_time: DateTime<Utc>,

    #[sea_orm(comment = "更新时间")]
    pub update_time: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let idgen = GLOBAL_IDGEN.lock().unwrap();
        Self {
            id: Set(idgen.next_id().unwrap()),
            create_time: Set(Utc::now()),
            update_time: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    // 插入或者更新前触发
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();
        let idgen = GLOBAL_IDGEN.lock().unwrap();

        // 如果没有设置id, 则默认给一个
        if self.id.is_not_set() {
            self.id = Set(idgen.next_id().unwrap());
        }

        // 新插入的则设置创建时间
        if insert {
            self.create_time = Set(now);
        }

        self.update_time = Set(now);

        Ok(self)
    }
}
