use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
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
}
