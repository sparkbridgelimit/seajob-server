use log::LevelFilter;
use sea_orm_migration::prelude::*;

use migration::sea_orm::Database;

#[async_std::main]
async fn main() -> Result<(), DbErr> {

    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _ = Database::connect(&database_url).await?;

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
