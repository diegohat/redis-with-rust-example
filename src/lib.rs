pub mod config;

pub use config::*;

use std::collections::HashMap;

use redis::{AsyncCommands, ErrorKind, RedisError, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl TryFrom<HashMap<String, String>> for Person {
    type Error = RedisError;

    fn try_from(hash: HashMap<String, String>) -> Result<Self, Self::Error> {
        let name = hash
            .get("name")
            .ok_or_else(|| RedisError::from((ErrorKind::TypeError, "Campo 'name' ausente")))?;
        let age_str = hash
            .get("age")
            .ok_or_else(|| RedisError::from((ErrorKind::TypeError, "Campo 'age' ausente")))?;
        let age: u32 = age_str.parse().map_err(|_| {
            RedisError::from((ErrorKind::TypeError, "Falha ao parsear 'age' como u32"))
        })?;
        Ok(Person {
            name: name.clone(),
            age,
        })
    }
}

pub async fn run_example() -> RedisResult<()> {
    let config = RedisConfig::builder()
        .host("localhost")
        .port(6380)
        .tls(true)
        .db(15)
        .build();

    let mut conn1 = config.connect().await?;
    let mut conn2 = conn1.clone();

    let person = Person {
        name: "João".to_string(),
        age: 30,
    };

    conn1
        .hset::<_, _, _, i64>("person:1", "name", &person.name)
        .await?;
    conn1
        .hset::<_, _, _, i64>("person:1", "age", person.age)
        .await?;

    let name: String = conn2.hget("person:1", "name").await?;
    let age: u32 = conn2.hget("person:1", "age").await?;

    let hash: HashMap<String, String> = conn2.hgetall("person:1").await?;

    let retrieved_person1: Person = hash.try_into()?;

    let retrieved_person2 = Person { name, age };
    println!("Pessoa recuperada 1: {:?}", retrieved_person1);
    println!("Pessoa recuperada 2: {:?}", retrieved_person2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_try_from_valid() {
        let mut hash = HashMap::new();
        hash.insert("name".to_string(), "João".to_string());
        hash.insert("age".to_string(), "30".to_string());

        let person: Person = hash.try_into().unwrap();
        assert_eq!(person.name, "João");
        assert_eq!(person.age, 30);
    }

    #[test]
    fn test_person_try_from_missing_name() {
        let mut hash = HashMap::new();
        hash.insert("age".to_string(), "30".to_string());

        let result: Result<Person, _> = hash.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_person_try_from_invalid_age() {
        let mut hash = HashMap::new();
        hash.insert("name".to_string(), "João".to_string());
        hash.insert("age".to_string(), "invalid".to_string());

        let result: Result<Person, _> = hash.try_into();
        assert!(result.is_err());
    }
}
