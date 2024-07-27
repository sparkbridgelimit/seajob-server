use sea_orm::{DbErr, EntityTrait};

use seajob_entity::prelude::JobDefine;

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
