use sea_orm_migration::prelude::*;

use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_table_sql = "
        CREATE TABLE job_prefer (
            id BIGINT PRIMARY KEY,
            job_define_id BIGINT NOT NULL,
            keyword VARCHAR(255) NOT NULL,
            city_code VARCHAR(255) NOT NULL,
            salary_range VARCHAR(255) NOT NULL,
            key_kills TEXT,
            exclude_company TEXT,
            exclude_job TEXT,
            channel VARCHAR(255) NOT NULL,
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
            "COMMENT ON COLUMN job_prefer.id IS 'Primary key';",
            "COMMENT ON COLUMN job_prefer.job_define_id IS '关联的计划ID';",
            "COMMENT ON COLUMN job_prefer.keyword IS '职位关键字';",
            "COMMENT ON COLUMN job_prefer.city_code IS '城市';",
            "COMMENT ON COLUMN job_prefer.salary_range IS '薪资范围';",
            "COMMENT ON COLUMN job_prefer.key_kills IS '技能关键字';",
            "COMMENT ON COLUMN job_prefer.exclude_company IS '不想要的公司';",
            "COMMENT ON COLUMN job_prefer.exclude_job IS '不想要的岗位';",
            "COMMENT ON COLUMN job_prefer.channel IS '渠道';",
            "COMMENT ON COLUMN job_prefer.create_time IS '创建时间';",
            "COMMENT ON COLUMN job_prefer.update_time IS '更新时间';",
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
            CREATE INDEX idx_prefer
            ON job_prefer (id, job_define_id);
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

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
