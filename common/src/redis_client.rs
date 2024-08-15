use std::env;
use std::sync::OnceLock;

use redis::aio::{MultiplexedConnection};
use redis::Client;

static REDIS_CLIENT: OnceLock<Client> = OnceLock::new();

/// 初始化 Redis 连接
pub async fn init_redis() {
    // 获取 Redis URL
    let redis_url = env::var("REDIS_URL").expect("缺失 REDIS_URL 环境变量");

    // 创建 Redis 客户端
    let client = Client::open(redis_url)
        .unwrap_or_else(|e| panic!("无法创建 Redis 客户端：{}", e));

    // 测试连接
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .unwrap_or_else(|e| panic!("Redis 连接失败：{}", e));

    let _: () = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .unwrap_or_else(|e| panic!("Redis 连接测试失败：{}", e));

    let _ = REDIS_CLIENT.set(client);
}

/// 获取多路复用Redis 连接
pub async fn multiplexed_conn() -> MultiplexedConnection {
    let client = REDIS_CLIENT
        .get()
        .unwrap_or_else(|| panic!("Redis 客户端未初始化"));

    client
        .get_multiplexed_async_connection()
        .await
        .unwrap_or_else(|e| panic!("无法获取 Redis 连接：{}", e))
}