use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::shop::repository::shop_repository::ShopRepository;

use crate::shop::repository::shop_repository_impl::ShopRepositoryImpl;
use crate::shop::service::request::free_card_request::FreeCardRequest;
use crate::shop::service::response::free_card_response::FreeCardResponse;
use crate::shop::service::shop_service::ShopService;

pub struct ShopServiceImpl {
    repository: Arc<AsyncMutex<ShopRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl ShopServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<ShopRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        ShopServiceImpl {
            repository,
            redis_in_memory_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopServiceImpl::new(
                            ShopRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ShopService for ShopServiceImpl {
    async fn free_card(&self, free_card_request: FreeCardRequest) -> FreeCardResponse {
        println!("ShopServiceImpl: free_card()");

        let shop_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_number_str = redis_repository_guard.get(free_card_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
        match account_unique_id {
            Ok(int_type_account_id) => {
                let result = shop_repository.add_free_cards(int_type_account_id).await;
                if result.is_ok() {
                    FreeCardResponse::new(result.unwrap())
                } else {
                    let empty_set = Vec::new();
                    FreeCardResponse::new(empty_set)
                }
            }
            Err(e) => {
                let empty_set = Vec::new();
                FreeCardResponse::new(empty_set)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_add_free_cards() {
        let shop_service_impl_mutex = ShopServiceImpl::get_instance();
        let shop_service_impl_mutex_guard = shop_service_impl_mutex.lock().await;

        let request = FreeCardRequest::new("redis_token_str".to_string());

        let result = shop_service_impl_mutex_guard.free_card(request).await;

        println!("result: {:?}", result);
    }
}