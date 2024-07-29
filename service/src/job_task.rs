use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;

use seajob_common::db;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;
use seajob_dto::req::job_task::{JobTaskEnd, JobTaskError, JobTaskList, JobTaskLog, JobTaskStart};
use seajob_entity::{job_contacted, job_task};
use seajob_entity::prelude::JobTask;

use crate::err::ServiceError;

pub struct JobTaskService;

impl JobTaskService {
    pub async fn list(req: JobTaskList) -> Result<Vec<job_task::Model>, ServiceError> {
        let txn = db::conn();

        let task_list = JobTask::find()
            .filter(job_task::Column::JobDefineId.eq(req.job_define_id))
            .all(txn)
            .await
            .map_err(ServiceError::DbError)?;

        if task_list.is_empty() {
            return Err(ServiceError::NotFoundError("job task is not found".to_string()));
        }

        Ok(task_list)
    }

    pub async fn start(req: JobTaskStart) -> Result<(), ServiceError> {
        let txn = db::conn();

        let job_task = JobTask::find()
            .filter(job_task::Column::JobDefineId.eq(req.job_define_id))
            .one(txn)
            .await?
            .ok_or(ServiceError::NotFoundError("Job task not found".to_string()))?;

        // 改成开始状态
        let mut to_update = job_task.into_active_model();
        to_update.status = Set("running".to_string());

        to_update.save(txn).await?;

        Ok(())
    }

    pub async fn log(req: JobTaskLog) -> Result<(), ServiceError> {
        let id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };

        let txn = db::conn();

        let job_task = JobTask::find_by_id(req.job_task_id)
            .one(txn)
            .await?
            .ok_or(ServiceError::NotFoundError("job_task not found".to_string()))?;

        job_contacted::ActiveModel {
            id: Set(id),
            job_task_id: Set(req.job_task_id),
            job_define_id: Set(job_task.job_define_id),
            user_id: Set(1),
            job_name: Set(req.job_name.unwrap_or_default()),
            job_link: Set(req.job_link.unwrap_or_default()),
            company: Set(req.company.unwrap_or_default()),
            boss_name: Set(req.boss_name.unwrap_or_default()),
            address: Set(req.address.unwrap_or_default()),
            salary_range: Set(serde_json::to_string(&req.salary_range).unwrap()),
            status: Set(job_contacted::Status::Unread),
            create_time: Default::default(),
            update_time: Default::default(),
        }
            .insert(txn)
            .await?;

        Ok(())
    }

    pub async fn error(req: JobTaskError) -> Result<(), ServiceError> {
        let txn = db::conn();

        let job_task = JobTask::find_by_id(req.id)
            .one(txn)
            .await?
            .ok_or(ServiceError::NotFoundError("job_task not found".to_string()))?;

        // 改成错误状态
        let mut to_update = job_task.into_active_model();
        to_update.status = Set("error".to_string());
        to_update.last_error = Set(req.error.unwrap_or_default());

        to_update.save(txn).await?;

        Ok(())
    }

    pub async fn end(req: JobTaskEnd) -> Result<(), ServiceError> {
        let txn = db::conn();

        let job_task = JobTask::find_by_id(req.id)
            .one(txn)
            .await?
            .ok_or(ServiceError::NotFoundError("job_task not found".to_string()))?;

        // 改成开始状态
        let mut to_update = job_task.into_active_model();
        to_update.status = Set("end".to_string());

        to_update.save(txn).await?;

        Ok(())
    }
}