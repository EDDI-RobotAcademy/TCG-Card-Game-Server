use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::env::env_detector::EnvDetector;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;

pub struct RedisInMemoryRepositoryImpl {
    connection: redis::Connection,
}

impl RedisInMemoryRepositoryImpl {
    fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/")
            .expect("Failed to connect to Redis");

        let mut connection = client
            .get_connection()
            .expect("Failed to connect to Redis with password");

        redis::cmd("AUTH")
            .arg(EnvDetector::get_redis_password())
            .query::<()>(&mut connection)
            .expect("Failed to authenticate to Redis");

        RedisInMemoryRepositoryImpl { connection }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RedisInMemoryRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>> =
                Arc::new(AsyncMutex::new(RedisInMemoryRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl RedisInMemoryRepository for RedisInMemoryRepositoryImpl {
    async fn set_permanent(&mut self, key: &str, value: &str) {
        println!("RedisInMemoryRepositoryImpl: set()");
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query(&mut self.connection)
            .expect("Failed to set key");
    }

    async fn set_with_expired_time(&mut self, key: &str, value: &str, expiry_seconds: Option<u32>) {
        println!("RedisInMemoryRepositoryImpl: set_with_expired_time()");

        if let Some(expiry_seconds) = expiry_seconds {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX").arg(expiry_seconds)
                .query::<()>(&mut self.connection)
                .expect("Failed to set key");
        }
    }

    async fn get(&mut self, key: &str) -> Option<String> {
        println!("RedisInMemoryRepositoryImpl: get()");
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query(&mut self.connection)
            .expect("Failed to get key");

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    // #[cfg(feature = "redis_integration_test")]
    async fn test_set_and_get_from_redis() {
        let redis_in_memory_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_in_memory_repository_gaurd = redis_in_memory_repository_mutex.lock().await;

        redis_in_memory_repository_gaurd.set_permanent("test_key", "test_value").await;

        let result = redis_in_memory_repository_gaurd.get("test_key").await;

        assert_eq!(result, Some("test_value".to_string()));
    }

    #[tokio::test]
    #[cfg(not(feature = "redis_integration_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}
