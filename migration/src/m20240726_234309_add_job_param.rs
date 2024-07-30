use sea_orm_migration::prelude::*;

use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table_sql = "
            CREATE TABLE job_param (
                id BIGINT PRIMARY KEY,
                job_define_id BIGINT NOT NULL,
                wt2_cookie VARCHAR(255) NOT NULL,
                interval INTEGER DEFAULT 5,
                timeout INTEGER DEFAULT 5,
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

        // 添加注释的 SQL 语句数组
        let comment_sql = [
            "COMMENT ON COLUMN job_param.id IS 'Primary key';",
            "COMMENT ON COLUMN job_param.job_define_id IS '关联的计划ID';",
            "COMMENT ON COLUMN job_param.wt2_cookie IS 'wt2_cookie';",
            "COMMENT ON COLUMN job_param.interval IS '投递间隔';",
            "COMMENT ON COLUMN job_param.timeout IS '超时时间';",
            "COMMENT ON COLUMN job_param.create_time IS '创建时间';",
            "COMMENT ON COLUMN job_param.update_time IS '更新时间';",
        ];

        // 逐个执行添加注释的 SQL 语句
        for comment_sql in &comment_sql {
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
            CREATE INDEX idx_job_param
            ON job_param (id, job_define_id);
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
