use sea_orm_migration::prelude::*;

use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table_sql = "
            CREATE TABLE job_define (
                id BIGINT PRIMARY KEY,
                user_id BIGINT NOT NULL,
                job_define_name VARCHAR(255) NOT NULL,
                job_define_desc VARCHAR(255),
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
            "COMMENT ON COLUMN job_define.id IS 'Primary key';",
            "COMMENT ON COLUMN job_define.user_id IS '关联的用户id';",
            "COMMENT ON COLUMN job_define.job_define_name IS '投递计划任务';",
            "COMMENT ON COLUMN job_define.job_define_desc IS '投递计划任务';",
            "COMMENT ON COLUMN job_define.create_time IS '创建时间';",
            "COMMENT ON COLUMN job_define.update_time IS '更新时间';",
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
            CREATE INDEX idx_job_define
            ON job_define (id, user_id);
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
