use redis::RedisResult;
use redis_with_rust_example::*;

#[tokio::main]
async fn main() -> RedisResult<()> {
    run_example().await
}
