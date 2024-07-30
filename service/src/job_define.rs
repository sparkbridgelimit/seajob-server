use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionTrait};

use seajob_common::db;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;
use seajob_dto::req::job_define::{
    JobDefineCreateRequest, JobDefineDetailRequest, JobDefineRunRequest,
};
use seajob_dto::res::job_define::JobDefineDetailResponse;
use seajob_entity::job_define::Model;
use seajob_entity::prelude::{JobDefine, JobParam, JobPrefer};
use seajob_entity::{job_define, job_param, job_prefer, job_task};

use crate::crud_service::{CRUDService, CRUDServiceImpl};
use crate::err::ServiceError;

pub struct JobDefineService {
    crud: CRUDServiceImpl<JobDefine>,
}

impl JobDefineService {
    pub fn new() -> Self {
        Self {
            crud: CRUDServiceImpl::<JobDefine>::new(),
        }
    }

    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<<JobDefine as EntityTrait>::Model>, DbErr> {
        self.crud.find_by_id(id).await
    }

    pub async fn find_all(&self) -> Result<Vec<<JobDefine as EntityTrait>::Model>, DbErr> {
        self.crud.find_all().await
    }
}

impl JobDefineService {
    pub async fn find_all_by_user(user_id: i64) -> Result<Vec<Model>, ServiceError> {
        let list = JobDefine::find()
            .filter(job_define::Column::UserId.eq(user_id))
            .all(db::conn())
            .await?;

        Ok(list)
    }

    pub async fn create(req: JobDefineCreateRequest) -> Result<bool, ServiceError> {
        let job_define_id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };
        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    // 插入用户
                    job_define::ActiveModel {
                        id: Set(job_define_id),
                        user_id: Set(req.user_id.unwrap()),
                        job_define_name: Set(req.job_define_name.unwrap()),
                        job_define_desc: Set(req.job_define_desc.unwrap()),
                        create_time: Default::default(),
                        update_time: Default::default(),
                    }
                        .insert(txn)
                        .await?;

                    // 创建 job_prefer
                    job_prefer::ActiveModel {
                        job_define_id: Set(job_define_id),
                        keyword: Set(req.keyword.unwrap_or_default()),
                        city_code: Set(req.city_code.unwrap_or_default()),
                        // 转["1", "1"]
                        salary_range: Set(serde_json::to_string(&req.salary_range).unwrap()),
                        // 转[""]
                        key_kills: Set(serde_json::to_string(&req.salary_range).unwrap()),
                        exclude_company: Set(serde_json::to_string(&req.exclude_company).unwrap()),
                        exclude_job: Set(serde_json::to_string(&req.exclude_job).unwrap()),
                        // 填充字段
                        ..Default::default()
                    }
                        .insert(txn)
                        .await?;

                    // 创建 job_param
                    job_param::ActiveModel {
                        job_define_id: Set(job_define_id),
                        greet_num: Set(req.greet_num),
                        ..Default::default()
                    }
                        .insert(txn)
                        .await?;

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))?;

        Ok(true)
    }

    // 运行一次任务, 根据参数每次
    pub async fn run(req: JobDefineRunRequest) -> Result<job_task::Model, ServiceError> {
        let id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };

        let txn = db::conn().begin().await?;

        // 查询运行参数
        JobParam::find()
            .filter(job_param::Column::JobDefineId.eq(req.job_define_id))
            .one(&txn)
            .await?
            .ok_or(ServiceError::ValidationError(
                "Job parameter not found".to_string(),
            ))?;

        // 插入新的 job_task 记录
        let new_job_task = job_task::ActiveModel {
            id: Set(id),
            job_define_id: Set(req.job_define_id.unwrap()),
            status: Set(String::from("pending")),
            wt2_cookie: Default::default(),
            target_num: Set(req.target_num.unwrap_or(0)),
            done_num: Set(0),
            last_error: Default::default(),
            create_time: Default::default(),
            update_time: Default::default(),
        };

        let inserted_job_task = new_job_task.insert(&txn).await?;
        // 提交事务
        txn.commit().await?;
        Ok(inserted_job_task)
    }

    pub async fn detail(
        req: JobDefineDetailRequest,
    ) -> Result<JobDefineDetailResponse, ServiceError> {
        // 查询job_define
        let jd = JobDefine::find()
            .filter(job_define::Column::Id.eq(req.job_define_id))
            .filter(job_define::Column::UserId.eq(req.user_id))
            .one(db::conn())
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job define not found".to_string()))?;

        // 查询job_prefer
        let jp = JobPrefer::find()
            .filter(job_prefer::Column::JobDefineId.eq(req.job_define_id))
            .one(db::conn())
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job prefer not found".to_string()))?;

        // 查询job_param
        let jpa = JobParam::find()
            .filter(job_param::Column::JobDefineId.eq(req.job_define_id))
            .one(db::conn())
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job param not found".to_string()))?;

        // 返回一个复合DTO
        let dto = JobDefineDetailResponse {
            job_define_id: req.job_define_id,
            job_define_name: jd.job_define_name,
            job_define_desc: jd.job_define_desc,
            keyword: jp.keyword,
            city_code: jp.city_code,
            salary_range: jp.salary_range,
            key_kills: jp.key_kills,
            exclude_company: jp.exclude_company,
            exclude_job: jp.exclude_job,
            interval: jpa.interval.unwrap_or_default(),
            timeout: jpa.timeout.unwrap_or_default(),
            greet_num: jpa.greet_num.unwrap_or_default(),
            wt2_cookie: jpa.wt2_cookie.unwrap_or_default(),
        };

        Ok(dto)
    }
}
