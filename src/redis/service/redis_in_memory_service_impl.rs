use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use serde_json::Value::Null;
use tokio::sync::Mutex as AsyncMutex;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;

pub struct RedisInMemoryServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl RedisInMemoryServiceImpl {
    pub fn new(redis_in_memory_repository:
               Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        RedisInMemoryServiceImpl {
            redis_in_memory_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RedisInMemoryServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RedisInMemoryServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RedisInMemoryServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl RedisInMemoryService for RedisInMemoryServiceImpl {
    async fn save_key_and_value(&mut self, key: &str, value: &str) {
        println!("RedisInMemoryServiceImpl: save_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        redis_in_memory_repository_guard.set_with_expired_time(key, value, Some(3600)).await;
    }
    async fn get_value_with_key(&mut self, key: &str) -> Option<String> {
        println!("RedisInMemoryServiceImpl: get_value_with_key()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        redis_in_memory_repository_guard.get(key).await
    }
    async fn delete_value_with_key(&mut self, key: &str) {
        println!("RedisInMemoryServiceImpl: delete_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        redis_in_memory_repository_guard.del(key).await
    }
    async fn save_daily_key_and_value(&mut self, key: &str, value: &str) {
        println!("RedisInMemoryServiceImpl: save_daily_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;

        let current_key = redis_in_memory_repository_guard.get(key).await;
        println!("current_key: {:?}", current_key);
        if Some(current_key) == Some(None) {
            redis_in_memory_repository_guard.set_with_expired_target_time(key, value).await;
            return
        }
        println!("Failed to set key.");
        return
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;
    use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;

    #[tokio::test]
    async fn test_save_daily_key_and_value() {
        let redis_in_memory_service_mutex = RedisInMemoryServiceImpl::get_instance();
        let mut redis_in_memory_service_mutex_guard = redis_in_memory_service_mutex.lock().await;

        redis_in_memory_service_mutex_guard.save_daily_key_and_value("test_key3", "test_value3").await;
    }
}