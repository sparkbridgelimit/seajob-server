use std::env;
use std::sync::OnceLock;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init_db() {
    let db_url = env::var("DATABASE_URL").expect("缺失 DATABASE_URL 环境变量");
    let mut opt = ConnectOptions::new(db_url);

    // 连接数据库
    opt.max_connections(1)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(8));

    let conn = Database::connect(opt)
        .await
        .unwrap_or_else(|e| panic!("数据库连接失败：{}", e));

    let _ = conn.ping().await.is_err_and(|e| panic!("数据库连接失败：{}", e));

    let _ = DB.set(conn);
}

pub fn conn() -> &'static DatabaseConnection {
    DB.get().unwrap_or_else(|| panic!("数据库连接未初始化"))
}