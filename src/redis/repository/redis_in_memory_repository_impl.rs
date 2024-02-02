use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use chrono::{Utc, Datelike, DateTime, TimeZone};

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

    async fn del(&mut self, key: &str) {
        println!("RedisInMemoryRepositoryImpl: del()");
        redis::cmd("DEL")
            .arg(key)
            .query::<()>(&mut self.connection)
            .expect("Failed to delete key");
    }

    async fn set_with_expired_target_time(&mut self, key: &str, value: &str) {
        println!("RedisInMemoryRepositoryImpl: set_with_expired_target_time()");

            let now = Utc::now;
            println!("now_time: {:?}", now());
            // 협정세계시 기준 (UTC) : 대한민국 시간은 UTC+9 (ex. UTC 14 시 59 분 59 초 = 대한민국 23 시 59 분 59 초)
            let target_time: DateTime<Utc> = Utc.with_ymd_and_hms(now().year(), now().month(), now().day(), 8, 44, 00).unwrap();
            println!("target_time: {:?}", target_time);
            let difference_time = target_time - now();
            let difference_seconds = difference_time.num_seconds();
            // 잔여시간을 시, 분, 초 으로 표현
            let hours = difference_seconds / 3600;
            let minutes = (difference_seconds % 3600) / 60;
            let remaining_seconds = difference_seconds % 60;
            println!("remaining_time: {:?} hours {:?} minutes {:?} sedonds", hours, minutes, remaining_seconds);

            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX").arg(difference_seconds)
                .query::<()>(&mut self.connection)
                .expect("Failed to set key");
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

        redis_in_memory_repository_gaurd.set_permanent("test_key2", "test_value2").await;

        let result = redis_in_memory_repository_gaurd.get("test_key2").await;
        println!("result value: {:?}", result);

        assert_eq!(result, Some("test_value2".to_string()));
    }

    #[tokio::test]
    // #[cfg(feature = "redis_integration_test")]
    async fn test_set_with_expired_target_time() {
        let redis_in_memory_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_in_memory_repository_gaurd = redis_in_memory_repository_mutex.lock().await;

        redis_in_memory_repository_gaurd.set_with_expired_target_time("test_key1", "test_value1").await;

        let result = redis_in_memory_repository_gaurd.get("test_key1").await;
        println!("result value: {:?}", result);
        assert_eq!(result, Some("test_value1".to_string()));
    }

    #[tokio::test]
    #[cfg(not(feature = "redis_integration_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}
