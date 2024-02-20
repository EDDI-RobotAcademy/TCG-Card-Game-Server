use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::account_point::repository::account_point_repository::AccountPointRepository;
use crate::account_point::repository::account_point_repository_impl::AccountPointRepositoryImpl;

use crate::shop::service::request::data_to_display_in_shop_request::DataToDisplayInShopRequest;
use crate::shop::service::response::data_to_display_in_shop_response::DataToDisplayInShopResponse;

use crate::shop::service::shop_service::ShopService;

pub struct ShopServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
}

impl ShopServiceImpl {
    pub fn new(
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,) -> Self {
        ShopServiceImpl {
            redis_in_memory_repository,
            account_point_repository,

        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            AccountPointRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
    async fn get_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
        account_unique_id
    }
}

#[async_trait]
impl ShopService for ShopServiceImpl {

    async fn data_to_display_in_shop(&self, data_to_display_in_shop_request: DataToDisplayInShopRequest) -> DataToDisplayInShopResponse {
        let account_unique_id = self.get_account_unique_id(data_to_display_in_shop_request.account_session_id()).await;

        let account_point_repository = self.account_point_repository.lock().await;
        let get_account_point = account_point_repository.find_by_account_id(account_unique_id).await.unwrap();

        DataToDisplayInShopResponse::new(get_account_point.unwrap().gold)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_add_free_cards() {
        // let shop_service_impl_mutex = ShopServiceImpl::get_instance();
        // let shop_service_impl_mutex_guard = shop_service_impl_mutex.lock().await;
        //
        // let request = GetCardDefaultRequest::new("qwer".to_string(), "Human".to_string(), true);
        //
        // let result = shop_service_impl_mutex_guard.get_specific_race_card_default(request).await;
        //
        // println!("result: {:?}", result);
    }
}