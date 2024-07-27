use sea_orm::{DbErr, EntityTrait};
use seajob_common::db;
use seajob_entity::job_contacted::Model;
use seajob_entity::prelude::ContactedJob;

pub struct ContactedJobService;

impl ContactedJobService {
    pub async fn find_by_id(contacted_job_id: i64) -> Result<Option<Model>, DbErr> {
        ContactedJob::find_by_id(contacted_job_id).one(db::conn()).await
    }

    pub async fn find_all() -> Result<Vec<Model>, DbErr> {
        ContactedJob::find().all(db::conn()).await
    }
}
