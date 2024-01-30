use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account_card::repository::account_card_repository::AccountCardRepository;

use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;
use crate::account_card::service::account_card_service::AccountCardService;
use crate::account_card::service::request::account_card_list_request::AccountCardListRequest;
use crate::account_card::service::response::account_card_list_response::AccountCardListResponse;


use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct AccountCardServiceImpl {
    repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl AccountCardServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        AccountCardServiceImpl {
            repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountCardServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountCardServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountCardServiceImpl::new(
                            AccountCardRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));

        }
        INSTANCE.clone()
    }
}
#[async_trait]
impl AccountCardService for AccountCardServiceImpl {

    async fn account_card_list(&self, account_card_list_request: AccountCardListRequest) -> AccountCardListResponse {
        println!("AccountCardServiceImpl: account_card_list()");

        let account_card_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_number_str = redis_repository_guard.get(account_card_list_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
        match account_unique_id {
            Ok(int_type_account_id) => {
                if let Some(card_list) = account_card_repository.get_card_list(int_type_account_id).await.unwrap() {
                    AccountCardListResponse::new(card_list)
                } else {
                    let empty_set = Vec::new();
                    AccountCardListResponse::new(empty_set)
                }
            }
            Err(e) => {
                println!("Card List Loading error : {}", e);
                let empty_set = Vec::new();
                AccountCardListResponse::new(empty_set)
            }
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_account_card_list() {
        let account_card_service_mutex = AccountCardServiceImpl::get_instance();
        let account_card_service_mutex_guard = account_card_service_mutex.lock().await;

        let account_card_list_request = AccountCardListRequest::new("redis_token_str".to_string());

        let result = account_card_service_mutex_guard.account_card_list(account_card_list_request).await;
        println!("{:?}", result.get_card_id_list());
    }

    #[tokio::test]
    #[cfg(not(feature = "account_card_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}
