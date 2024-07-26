use sea_orm::{DbConn, DbErr, EntityTrait};
use seajob_entity::contacted_job::Model;
use seajob_entity::prelude::ContactedJob;

pub struct ContactedJobService;

impl ContactedJobService {
    pub async fn find_by_id(db: &DbConn, contacted_job_id: String) -> Result<Option<Model>, DbErr> {
        ContactedJob::find_by_id(contacted_job_id).one(db).await
    }

    pub async fn find_all(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        ContactedJob::find().all(db).await
    }
}
