use redis::{AsyncCommands, RedisResult};
use redis_with_rust_example::{Person, RedisConfig};
use std::collections::HashMap;

#[tokio::test]
async fn test_redis_connection() -> RedisResult<()> {
    let mut conn = RedisConfig::builder().connect().await?;

    conn.set::<_, _, ()>("test_key", "test_value").await?;
    let value: String = conn.get("test_key").await?;

    assert_eq!(value, "test_value");
    Ok(())
}

#[tokio::test]
async fn test_tls_connection() -> RedisResult<()> {
    let mut conn = RedisConfig::builder()
        .port(6380)
        .tls(true)
        .connect()
        .await?;

    conn.set::<_, _, ()>("tls_test", 100).await?;
    let value: isize = conn.get("tls_test").await?;

    assert_eq!(value, 100);
    Ok(())
}

#[tokio::test]
async fn test_builder_reuse() -> RedisResult<()> {
    let config = RedisConfig::builder().host("127.0.0.1").db(15).build();

    let mut conn1 = config.connect().await?;
    let mut conn2 = config.clone().connect().await?;

    conn1.set::<_, _, ()>("reuse_test", "value").await?;
    let value: String = conn2.get("reuse_test").await?;

    assert_eq!(value, "value");
    Ok(())
}

#[tokio::test]
async fn test_person_hash_operations() -> RedisResult<()> {
    let config = RedisConfig::builder()
        .host("localhost")
        .port(6380)
        .tls(true)
        .db(15)
        .build();

    let mut conn = config.connect().await?;

    let person = Person {
        name: "Test".to_string(),
        age: 25,
    };

    conn.hset::<_, _, _, i64>("test_person", "name", &person.name)
        .await?;
    conn.hset::<_, _, _, i64>("test_person", "age", person.age)
        .await?;

    let hash: HashMap<String, String> = conn.hgetall("test_person").await?;

    let retrieved: Person = hash.try_into()?;

    assert_eq!(retrieved.name, "Test");
    assert_eq!(retrieved.age, 25);

    Ok(())
}
