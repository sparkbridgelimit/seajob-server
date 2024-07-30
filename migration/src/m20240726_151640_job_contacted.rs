use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table_sql = "
            CREATE TABLE job_contacted (
                id BIGINT PRIMARY KEY,
                job_define_id BIGINT NOT NULL,
                job_task_id BIGINT NOT NULL,
                user_id BIGINT NOT NULL,
                job_name VARCHAR(255) NOT NULL,
                job_link VARCHAR(255) NOT NULL,
                company VARCHAR(255) NOT NULL,
                boss_name VARCHAR(255) NOT NULL,
                address VARCHAR(255) NOT NULL,
                salary_range VARCHAR(255) NOT NULL,
                status VARCHAR(16) NOT NULL,
                create_time TIMESTAMP WITH TIME ZONE NOT NULL,
                update_time TIMESTAMP WITH TIME ZONE NOT NULL
            );
        ";

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                create_table_sql.to_string(),
            ))
            .await?;

        let comment_sqls = [
            "COMMENT ON COLUMN job_contacted.id IS 'Primary key';",
            "COMMENT ON COLUMN job_contacted.job_define_id IS '关联的JobDefineId';",
            "COMMENT ON COLUMN job_contacted.job_task_id IS '关联的任务id';",
            "COMMENT ON COLUMN job_contacted.user_id IS '关联的用户id';",
            "COMMENT ON COLUMN job_contacted.job_name IS '岗位名称';",
            "COMMENT ON COLUMN job_contacted.job_link IS '岗位链接';",
            "COMMENT ON COLUMN job_contacted.company IS '公司名称';",
            "COMMENT ON COLUMN job_contacted.boss_name IS '招聘者';",
            "COMMENT ON COLUMN job_contacted.address IS '地点';",
            "COMMENT ON COLUMN job_contacted.salary_range IS '薪资范围: [10, 20]';",
            "COMMENT ON COLUMN job_contacted.status IS '状态';",
            "COMMENT ON COLUMN job_contacted.create_time IS '创建时间';",
            "COMMENT ON COLUMN job_contacted.update_time IS '更新时间';",
        ];

        for comment_sql in &comment_sqls {
            manager
                .get_connection()
                .execute(Statement::from_string(
                    DbBackend::Postgres,
                    comment_sql.to_string(),
                ))
                .await?;
        }

        // 创建索引的原生 SQL
        let create_index_sql = "
            CREATE INDEX idx_job_contacted
            ON job_contacted (id, job_define_id, job_task_id, user_id);
        ";

        // 执行创建索引的原生 SQL
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                create_index_sql.to_string(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
