use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::async_trait::async_trait;
use serde::Serialize;
use strum_macros::{EnumString, Display};
use seajob_common::id_gen::id_generator::{GLOBAL_IDGEN};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, EnumString, Display, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(16))")]
pub enum Status {
    #[sea_orm(string_value = "unread")]
    Unread,
    #[sea_orm(string_value = "read")]
    Read,
    #[sea_orm(string_value = "replied")]
    Replied,
    #[sea_orm(string_value = "apply_resume")]
    ApplyResume,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "job_contacted")]
pub struct Model {
    #[sea_orm(primary_key, comment = "Primary key")]
    pub id: i64,

    #[sea_orm(comment = "关联的JobDefineId")]
    pub job_define_id: i64,

    #[sea_orm(comment = "关联的任务id")]
    pub job_task_id: i64,

    #[sea_orm(comment = "关联的用户id")]
    pub user_id: i64,

    #[sea_orm(comment = "岗位名称")]
    pub job_name: String,

    #[sea_orm(comment = "岗位链接")]
    pub job_link: String,

    #[sea_orm(comment = "公司名称")]
    pub company: String,

    #[sea_orm(comment = "招聘者")]
    pub boss_name: String,

    #[sea_orm(comment = "地点")]
    pub address: String,

    #[sea_orm(comment = "薪资范围: [10, 20]")]
    pub salary_range: String,

    #[sea_orm(comment = "状态")]
    pub status: Status,

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
