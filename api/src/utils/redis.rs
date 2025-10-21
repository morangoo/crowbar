use redis::{Client, RedisResult};

pub async fn get_redis_conn_async() -> RedisResult<redis::aio::MultiplexedConnection> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_multiplexed_tokio_connection().await
}