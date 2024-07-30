use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use seajob_common::db;
use seajob_dto::req::job_contacted::{JobContactedDefine, JobContactedTaskReq, JobContactedUser};
use seajob_entity::job_contacted::Model;
use seajob_entity::prelude::JobContacted;

use crate::err::ServiceError;

pub struct JobContactedService;

impl JobContactedService {
    // 查询用户所有已沟通岗位
    pub async fn find_all_by_user(req: JobContactedUser) -> Result<Vec<Model>, ServiceError> {
        let jc = JobContacted::find()
            .filter(seajob_entity::job_contacted::Column::UserId.eq(req.user_id))
            .all(db::conn())
            .await?;

        Ok(jc)
    }

    // 根据job_define查询
    pub async fn find_all_by_job_define(
        req: JobContactedDefine,
    ) -> Result<Vec<Model>, ServiceError> {
        let jc = JobContacted::find()
            .filter(seajob_entity::job_contacted::Column::JobDefineId.eq(req.job_define_id))
            .all(db::conn())
            .await?;

        Ok(jc)
    }

    pub async fn find_all_by_job_task(
        req: JobContactedTaskReq,
    ) -> Result<Vec<Model>, ServiceError> {
        let jc = JobContacted::find()
            .filter(seajob_entity::job_contacted::Column::JobTaskId.eq(req.job_task_id))
            .all(db::conn())
            .await?;

        Ok(jc)
    }
}
