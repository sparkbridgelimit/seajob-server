use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryOrder,
    QuerySelect, TransactionTrait,
};

use chrono::Utc;
use seajob_common::db;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;
use seajob_dto::req::job_define::{JobDefineCreateRequest, JobDefineDelete, JobDefineDetailRequest, JobDefineRunRequest, JobDefineSaveCookieRequest, JobDefineUpdateRequest};
use seajob_dto::res::job_define::{JobDefineDetailResponse, JobDefineRunResponse};
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
            .order_by_desc(job_define::Column::CreateTime)
            .all(db::conn())
            .await?;

        Ok(list)
    }

    pub async fn create(req: JobDefineCreateRequest, user_id: i64) -> Result<bool, ServiceError> {
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
                        user_id: Set(user_id),
                        job_define_name: Set(req.job_define_name.unwrap_or_default()),
                        job_define_desc: Set(req.job_define_desc.unwrap_or_default()),
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
                        key_kills: Set(serde_json::to_string(&req.key_kills).unwrap()),
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
                        hello_text: Set(req.hello_text),
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

    pub async fn update(req: JobDefineUpdateRequest) -> Result<bool, ServiceError> {
        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    // 查询
                    let jd = JobDefine::find_by_id(req.id)
                        .one(txn)
                        .await?
                        .ok_or_else(|| {
                            ServiceError::NotFoundError("Job define not found".to_string())
                        })?;

                    let job_define_id = jd.id;

                    let mut jd_active_model: job_define::ActiveModel = jd.into();

                    if let Some(job_define_name) = req.job_define_name {
                        jd_active_model.job_define_name = Set(job_define_name);
                    }

                    if let Some(job_define_desc) = req.job_define_desc {
                        jd_active_model.job_define_desc = Set(job_define_desc);
                    }
                    jd_active_model.update_time = Set(Utc::now().into());
                    // 保存更新
                    jd_active_model.update(txn).await?;

                    // 查询job_prefer
                    let mut jp: job_prefer::ActiveModel = JobPrefer::find()
                        .filter(job_prefer::Column::JobDefineId.eq(job_define_id))
                        .one(txn)
                        .await?
                        .ok_or_else(|| {
                            ServiceError::NotFoundError("Job prefer not found".to_string())
                        })?
                        .into();

                    if let Some(keyword) = req.keyword {
                        jp.keyword = Set(keyword);
                    }
                    if let Some(city_code) = req.city_code {
                        jp.city_code = Set(city_code);
                    }
                    if let Some(salary_range) = req.salary_range {
                        jp.salary_range = Set(serde_json::to_string(&salary_range).unwrap());
                    }
                    if let Some(key_kills) = req.key_kills {
                        jp.key_kills = Set(serde_json::to_string(&key_kills).unwrap());
                    }
                    if let Some(exclude_company) = req.exclude_company {
                        jp.exclude_company = Set(serde_json::to_string(&exclude_company).unwrap());
                    }
                    if let Some(exclude_job) = req.exclude_job {
                        jp.exclude_job = Set(serde_json::to_string(&exclude_job).unwrap());
                    }

                    jp.update_time = Set(Utc::now().into());
                    // 保存更新
                    jp.update(txn).await?;

                    // 查询job_param
                    let mut jpa: job_param::ActiveModel = JobParam::find()
                        .filter(job_param::Column::JobDefineId.eq(job_define_id))
                        .one(txn)
                        .await?
                        .ok_or_else(|| {
                            ServiceError::NotFoundError("Job param not found".to_string())
                        })?
                        .into();

                    if let Some(hello_text) = req.hello_text {
                        jpa.hello_text = Set(Some(hello_text));
                    }
                    if let Some(wt2_cookie) = req.wt2_cookie {
                        jpa.wt2_cookie = Set(Some(wt2_cookie));
                    }
                    jpa.update_time = Set(Utc::now().into());
                    // 保存更新
                    jpa.update(txn).await?;

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))?;

        Ok(true)
    }

    // 运行一次任务, 根据参数每次
    pub async fn run(req: JobDefineRunRequest, user_id: i64) -> Result<JobDefineRunResponse, ServiceError> {
        let id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };

        let txn = db::conn().begin().await?;

        // 查询job_define
        let jd = JobDefine::find()
            .filter(job_define::Column::Id.eq(req.job_define_id))
            .filter(job_define::Column::UserId.eq(user_id))
            .one(&txn)
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job define not found".to_string()))?;

        // 查询job_prefer
        let jp = JobPrefer::find()
            .filter(job_prefer::Column::JobDefineId.eq(req.job_define_id))
            .one(&txn)
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job prefer not found".to_string()))?;

        // 查询job_param
        let jpa = JobParam::find()
            .filter(job_param::Column::JobDefineId.eq(req.job_define_id))
            .one(&txn)
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Job param not found".to_string()))?;


        // 插入新的 job_task 记录
        let new_job_task = job_task::ActiveModel {
            id: Set(id),
            job_define_id: Set(req.job_define_id),
            status: Set(String::from("pending")),
            target_num: Set(req.target_num),
            done_num: Set(0),
            last_error: Default::default(),
            create_time: Default::default(),
            update_time: Default::default(),
        };

        new_job_task.insert(&txn).await?;

        let dto = JobDefineRunResponse {
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
            wt2_cookie: jpa.wt2_cookie.unwrap_or_default(),
            hello_text: jpa.hello_text.unwrap_or_default(),
            target_num: req.target_num,
        };

        // 提交事务
        txn.commit().await?;
        Ok(dto)
    }

    pub async fn detail(
        req: JobDefineDetailRequest,
        user_id: i64,
    ) -> Result<JobDefineDetailResponse, ServiceError> {
        // 查询job_define
        let jd = JobDefine::find()
            .filter(job_define::Column::Id.eq(req.job_define_id))
            .filter(job_define::Column::UserId.eq(user_id))
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
            wt2_cookie: jpa.wt2_cookie.unwrap_or_default(),
            hello_text: jpa.hello_text.unwrap_or_default(),
        };

        Ok(dto)
    }

    pub async fn delete(req: JobDefineDelete) -> Result<bool, ServiceError> {
        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    JobDefine::delete_by_id(req.job_define_id).exec(txn).await?;

                    let jpa = JobParam::find()
                        .select_only()
                        .column(job_param::Column::Id)
                        .filter(job_param::Column::JobDefineId.eq(req.job_define_id))
                        .one(txn)
                        .await;

                    if let Ok(Some(j)) = jpa {
                        j.delete(txn).await?;
                    }

                    let jp = JobPrefer::find()
                        .select_only()
                        .column(job_prefer::Column::Id)
                        .filter(job_prefer::Column::JobDefineId.eq(req.job_define_id))
                        .one(txn)
                        .await;

                    if let Ok(Some(p)) = jp {
                        p.delete(txn).await?;
                    }

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))?;
        Ok(true)
    }

    pub async fn save_cookie(req: JobDefineSaveCookieRequest) -> Result<bool, ServiceError> {
        db::conn()
            .transaction::<_, _, ServiceError>(|txn| {
                Box::pin(async move {
                    // 查询job_param
                    let mut jpa: job_param::ActiveModel = JobParam::find()
                        .filter(job_param::Column::JobDefineId.eq(req.job_define_id))
                        .one(txn)
                        .await?
                        .ok_or_else(|| {
                            ServiceError::NotFoundError("Job param not found".to_string())
                        })?
                        .into();

                    if let Some(cookie) = req.cookie {
                        jpa.wt2_cookie = Set(Some(cookie));
                    }

                    jpa.update_time = Set(Utc::now().into());

                    // 保存更新
                    jpa.update(txn).await?;

                    Ok(true)
                })
            })
            .await
            .map_err(|e| ServiceError::TransactionError(Box::new(e)))?;

        Ok(true)
    }
}
