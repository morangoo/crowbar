use redis::{Client, RedisResult};
use crate::utils::config::get_redis_url;

pub async fn get_redis_conn_async() -> RedisResult<redis::aio::MultiplexedConnection> {
    let client = Client::open(get_redis_url())?;
    client.get_multiplexed_tokio_connection().await
}