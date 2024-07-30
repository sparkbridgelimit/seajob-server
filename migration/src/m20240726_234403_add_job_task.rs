use sea_orm_migration::prelude::*;

use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table_sql = "
            CREATE TABLE job_task (
                id BIGINT PRIMARY KEY,
                job_define_id BIGINT NOT NULL,
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
        let comment_sqls = [
            "COMMENT ON COLUMN job_task.id IS 'Primary key';",
            "COMMENT ON COLUMN job_task.job_define_id IS '关联的计划ID';",
            "COMMENT ON COLUMN job_task.create_time IS '创建时间';",
            "COMMENT ON COLUMN job_task.update_time IS '更新时间';",
        ];

        // 逐个执行添加注释的 SQL 语句
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
            CREATE INDEX idx_task
            ON job_task (id, job_define_id);
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

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
