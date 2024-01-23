use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::account_deck::entity::account_deck::AccountDeck;
use crate::account_deck::repository::account_deck_repository::AccountDeckRepository;
use crate::account_deck::repository::account_deck_repository_impl::AccountDeckRepositoryImpl;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;


pub struct AccountDeckServiceImpl {
    repository: Arc<AsyncMutex<AccountDeckRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl AccountDeckServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountDeckRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        AccountDeckServiceImpl {
            repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountDeckServiceImpl::new(
                            AccountDeckRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckService for AccountDeckServiceImpl {
    async fn account_deck_register(&self, account_deck_register_request: AccountDeckRegisterRequest) -> AccountDeckRegisterResponse {
        println!("AccountDeckServiceImpl: account_deck_register()");

        let account_deck_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_number_str = redis_repository_guard.get(account_deck_register_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
        match account_unique_id {
            Ok(int_type_account_id) => {
                let account_deck = AccountDeck::new(int_type_account_id, account_deck_register_request.deck_name());
                match account_deck {
                    Ok(new_deck) => {
                        let result = account_deck_repository.save(new_deck).await;
                        AccountDeckRegisterResponse::new(true)
                    }
                    Err(e) => {
                        AccountDeckRegisterResponse::new(false)
                    }
                }
            }
            Err(e) => {
                AccountDeckRegisterResponse::new(false)
            }
        }
        // TODO: 덱이 계정당 최대 6개만 생성되어야 하므로, 계정의 덱 개수를 카운팅하는 작업 필요
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

    #[tokio::test]
    async fn test_deck_registration() {
        let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
        let mut account_deck_service_mutex_guard = account_deck_service_mutex.lock().await;

        // let redis_in_memory_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        // let mut redis_in_memory_repository_guard = redis_in_memory_repository_mutex.lock().await;
        //
        // redis_in_memory_repository_guard.set_permanent("redis_token_str", "1").await;

        let redis_token_str = "redis_token_str";
        let sample_deck_name = "너무 어렵죠?";

        let account_deck_register_request = AccountDeckRegisterRequest::new(redis_token_str.to_string(), sample_deck_name.to_string());

        let result = account_deck_service_mutex_guard.account_deck_register(account_deck_register_request).await;

        assert_eq!(true, result.get_is_success());
    }
    #[tokio::test]
    #[cfg(not(feature = "deck_registration_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}