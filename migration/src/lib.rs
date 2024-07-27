pub use sea_orm_migration::prelude::*;

mod m20240726_151640_job_contacted;
mod m20240726_234256_add_job_define;
mod m20240726_234309_add_job_param;
mod m20240726_234354_add_job_prefer;
mod m20240726_234403_add_job_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240726_151640_job_contacted::Migration),
            Box::new(m20240726_234256_add_job_define::Migration),
            Box::new(m20240726_234309_add_job_param::Migration),
            Box::new(m20240726_234354_add_job_prefer::Migration),
            Box::new(m20240726_234403_add_job_task::Migration),
        ]
    }
}
