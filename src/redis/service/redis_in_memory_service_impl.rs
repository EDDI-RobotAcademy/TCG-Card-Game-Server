use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::redis::service::request::delete_value_with_key_request::DeleteValueWithKeyRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::redis::service::request::save_daily_key_and_value_request::SaveDailyKeyAndValueRequest;
use crate::redis::service::request::save_key_and_value_request::SaveKeyAndValueRequest;

use crate::redis::service::response::delete_value_with_key_response::DeleteValueWithKeyResponse;
use crate::redis::service::response::get_value_with_key_response::GetValueWithKeyResponse;
use crate::redis::service::response::save_daily_key_and_value_response::SaveDailyKeyAndValueResponse;
use crate::redis::service::response::save_key_and_value_response::SaveKeyAndValueResponse;

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
    async fn save_key_and_value(&self, save_key_and_value_request: SaveKeyAndValueRequest) -> SaveKeyAndValueResponse {
        println!("RedisInMemoryServiceImpl: save_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        let result_redis = redis_in_memory_repository_guard.set_with_expired_time(save_key_and_value_request.key(), save_key_and_value_request.value(), Some(3600)).await;
        if Some(result_redis).is_some() {
            return SaveKeyAndValueResponse::new(true)
        }
        return SaveKeyAndValueResponse::new(false)
    }
    async fn get_value_with_key(&self, get_value_with_key_request: GetValueWithKeyRequest) -> GetValueWithKeyResponse {
        println!("RedisInMemoryServiceImpl: get_value_with_key()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        let result_redis = redis_in_memory_repository_guard.get(get_value_with_key_request.key()).await;
        if result_redis.clone().is_some() {
            return GetValueWithKeyResponse::new(result_redis.unwrap_or("".to_string()))
        }
        println!("The key does not exist.");
        return GetValueWithKeyResponse::new("".to_string())
    }
    async fn delete_value_with_key(&self, delete_value_with_key_request: DeleteValueWithKeyRequest) -> DeleteValueWithKeyResponse {
        println!("RedisInMemoryServiceImpl: delete_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;
        let result_redis = redis_in_memory_repository_guard.del(delete_value_with_key_request.key()).await;
        if Some(result_redis).is_none() {
            return DeleteValueWithKeyResponse::new(true)
        }
        return DeleteValueWithKeyResponse::new(false)
    }
    async fn save_daily_key_and_value(&self, save_daily_key_and_value_request: SaveDailyKeyAndValueRequest) -> SaveDailyKeyAndValueResponse {
        println!("RedisInMemoryServiceImpl: save_daily_key_and_value()");

        let mut redis_in_memory_repository_guard = self.redis_in_memory_repository.lock().await;

        let current_key = redis_in_memory_repository_guard.get(save_daily_key_and_value_request.key()).await;
        println!("current_key: {:?}", current_key);
        if Some(current_key) == Some(None) {
            redis_in_memory_repository_guard.set_with_expired_target_time(save_daily_key_and_value_request.key(), save_daily_key_and_value_request.value()).await;
            return SaveDailyKeyAndValueResponse::new(true)
        }
        println!("Failed to set key.");
        return SaveDailyKeyAndValueResponse::new(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;
    use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;

    #[tokio::test]
    async fn test_save_key_and_value() {
        let redis_in_memory_service_mutex = RedisInMemoryServiceImpl::get_instance();
        let mut redis_in_memory_service_mutex_guard = redis_in_memory_service_mutex.lock().await;

        let test_account = SaveKeyAndValueRequest::new("test_key5", "test_value5");

        redis_in_memory_service_mutex_guard.save_key_and_value(test_account).await;
    }

    #[tokio::test]
    async fn test_get_value_with_key() {
        let redis_in_memory_service_mutex = RedisInMemoryServiceImpl::get_instance();
        let mut redis_in_memory_service_mutex_guard = redis_in_memory_service_mutex.lock().await;

        let test_account = GetValueWithKeyRequest::new("test_key5");

        let result_redis = redis_in_memory_service_mutex_guard.get_value_with_key(test_account).await;
        println!("get_redis_value: {:?}", result_redis);
    }

    #[tokio::test]
    async fn test_delete_value_with_key() {
        let redis_in_memory_service_mutex = RedisInMemoryServiceImpl::get_instance();
        let mut redis_in_memory_service_mutex_guard = redis_in_memory_service_mutex.lock().await;

        let test_account = DeleteValueWithKeyRequest::new("test_key5");

        let result_redis = redis_in_memory_service_mutex_guard.delete_value_with_key(test_account).await;
    }

    #[tokio::test]
    async fn test_save_daily_key_and_value() {
        let redis_in_memory_service_mutex = RedisInMemoryServiceImpl::get_instance();
        let mut redis_in_memory_service_mutex_guard = redis_in_memory_service_mutex.lock().await;

        let test_account = SaveDailyKeyAndValueRequest::new("test_key3", "test_value3");

        redis_in_memory_service_mutex_guard.save_daily_key_and_value(test_account).await;
    }
}