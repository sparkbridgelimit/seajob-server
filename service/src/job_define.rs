use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, NotSet, TransactionError, TransactionTrait};
use sea_orm::ActiveValue::Set;
use log::info;
use seajob_common::db;
use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;
use seajob_dto::req::job_define_create::JobDefineCreateRequest;
use seajob_entity::prelude::{JobDefine};
use seajob_entity::{job_define};

use crate::crud_service::{CRUDService, CRUDServiceImpl};


pub struct JobDefineService {
    crud: CRUDServiceImpl<JobDefine>,
}

impl JobDefineService {
    pub fn new() -> Self {
        Self {
            crud: CRUDServiceImpl::<JobDefine>::new(),
        }
    }

    pub async fn find_by_id(&self, contacted_job_id: i64) -> Result<Option<<JobDefine as EntityTrait>::Model>, DbErr> {
        self.crud.find_by_id(contacted_job_id).await
    }

    pub async fn find_all(&self) -> Result<Vec<<JobDefine as EntityTrait>::Model>, DbErr> {
        self.crud.find_all().await
    }
}

impl JobDefineService {
    pub async fn create(req: JobDefineCreateRequest) -> Result<bool, TransactionError<DbErr>> {
        let job_define_id = {
            let id_gen = GLOBAL_IDGEN.lock().unwrap();
            id_gen.next_id().unwrap()
        };
        db::conn().transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                info!("Starting transaction to create job_define");
                // 创建 job_define
                let j = job_define::ActiveModel {
                    id: Set(job_define_id),
                    user_id: Set(1),
                    job_define_name: Set(req.job_define_name.unwrap_or_default()),
                    job_define_desc: Set(req.job_define_desc.unwrap_or_default()),
                    greet_num: Set(req.greet_num.unwrap_or_default()),
                    create_time: NotSet,
                    update_time: NotSet,
                };
                info!("111");
                j.insert(txn)
                    .await?;
                info!("222");

                //
                // // 创建 job_prefer
                // job_prefer::ActiveModel {
                //     job_define_id: Set(job_define_id),
                //     keyword: Set(req.keyword.unwrap_or_default()),
                //     city_code: Set(req.city_code.unwrap_or_default()),
                //     // 转["1", "1"]
                //     salary_range: Set(serde_json::to_string(&req.salary_range).unwrap()),
                //     // 转[""]
                //     key_kills: Set(serde_json::to_string(&req.salary_range).unwrap()),
                //     exclude_company: Set(serde_json::to_string(&req.exclude_company).unwrap()),
                //     exclude_job: Set(serde_json::to_string(&req.exclude_job).unwrap()),
                //     // 填充字段
                //     ..Default::default()
                // }
                //     .save(txn)
                //     .await?;

                // 创建 job_param
                // job_param::ActiveModel {
                //     job_define_id: Set(job_define_id),
                //     ..Default::default()
                // }
                //     .save(txn)
                //     .await?;

                Ok(true)
            })
        })
            .await
    }
}
