use deadpool_redis::{Config, Connection, Pool, Runtime};
use std::env;
use std::sync::OnceLock;

static REDIS_POOL: OnceLock<Pool> = OnceLock::new();

/// 初始化 Redis 连接池
pub async fn init_redis_pool() {
    // 获取 Redis URL
    let redis_url = env::var("REDIS_URL").expect("缺失 REDIS_URL 环境变量");

    // 使用配置创建 Redis 连接池
    let cfg = Config::from_url(redis_url);

    // 创建 Redis 连接池
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("无法创建 Redis 连接池");

    // 设置全局的 Redis 连接池
    REDIS_POOL.set(pool).expect("无法设置 Redis 连接池");
}

/// 获取多路复用Redis 连接
pub async fn multiplexed_conn() -> Connection {
    let pool = REDIS_POOL.get().expect("连接池未初始化");
    let conn = pool.get().await.expect("无法获取 Redis 连接");

    conn
}
