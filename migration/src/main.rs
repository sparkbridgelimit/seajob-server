use log::LevelFilter;
use sea_orm_migration::prelude::*;
use migration::sea_orm::{Database, DatabaseBackend, Statement};

#[async_std::main]
async fn main() -> Result<(), DbErr> {

    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();


    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await?;

    // 清理现有的预备语句
    // db.execute(Statement::from_string(
    //     DatabaseBackend::Postgres,
    //     "DEALLOCATE ALL;".to_string(),
    // ))
    // .await?;

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
