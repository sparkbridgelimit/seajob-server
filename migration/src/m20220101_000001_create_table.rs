use sea_orm_migration::prelude::*;
use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(ContactedJob::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ContactedJob::ContactedJobId)
                        .char_len(26)
                        .not_null()
                        .primary_key()
                        .comment("Primary key")
                )
                .col(
                    ColumnDef::new(ContactedJob::JobDefineId)
                        .char_len(26)
                        .not_null()
                        .comment("关联的JobDefineId")
                )
                .col(
                    ColumnDef::new(ContactedJob::JobTaskId)
                        .char_len(26)
                        .not_null()
                        .comment("关联的任务id")
                )
                .col(
                    ColumnDef::new(ContactedJob::UserId)
                        .char_len(26)
                        .not_null()
                        .comment("关联的用户id")
                )
                .col(
                    ColumnDef::new(ContactedJob::JobName)
                        .string_len(255)
                        .comment("岗位名称")
                )
                .col(
                    ColumnDef::new(ContactedJob::JobLink)
                        .string_len(255)
                        .comment("岗位链接")
                )
                .col(
                    ColumnDef::new(ContactedJob::Company)
                        .string_len(255)
                        .comment("公司名称")
                )
                .col(
                    ColumnDef::new(ContactedJob::BossName)
                        .string_len(255)
                        .comment("招聘者")
                )
                .col(
                    ColumnDef::new(ContactedJob::Address)
                        .string_len(255)
                        .comment("地点")
                )
                .col(
                    ColumnDef::new(ContactedJob::SalaryRange)
                        .string_len(255)
                        .comment("薪资范围")
                )
                .col(
                    ColumnDef::new(ContactedJob::Channel)
                        .string_len(16)
                        .not_null()
                        .comment("渠道")
                )
                .col(
                    ColumnDef::new(ContactedJob::Status)
                        .string_len(16)
                        .not_null()
                        .default("unread")
                        .comment("状态")
                )
                .col(
                    ColumnDef::new(ContactedJob::CreateTime)
                        .timestamp_with_time_zone()
                        .not_null()
                        .comment("创建时间")
                )
                .col(
                    ColumnDef::new(ContactedJob::UpdateTime)
                        .timestamp_with_time_zone()
                        .not_null()
                        .comment("更新时间")
                )
                .to_owned()
        )
            .await?;

        // 创建索引的原生 SQL
        let create_index_sql = "
            CREATE INDEX idx_job_define_id_job_task_id_user_id
            ON contacted_job (job_define_id, job_task_id, user_id);
        ";

        // 执行创建索引的原生 SQL
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                create_index_sql.to_string(),
            ))
            .await?;

        Ok(()) // All good!
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
                .table(ContactedJob::Table)
                .to_owned()
        )
            .await?;

        // 删除索引
        let drop_index_sql = "
            DROP INDEX IF EXISTS idx_job_define_id_job_task_id_user_id;
        ";

        // 执行删除索引的原生 SQL
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                drop_index_sql.to_string(),
            ))
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum ContactedJob {
    Table,
    ContactedJobId,
    JobDefineId,
    JobTaskId,
    UserId,
    JobName,
    JobLink,
    Company,
    BossName,
    Address,
    SalaryRange,
    Channel,
    Status,
    CreateTime,
    UpdateTime,
}
