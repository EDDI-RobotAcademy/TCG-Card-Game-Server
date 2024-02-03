use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::account_deck::entity::account_deck::AccountDeck;
use crate::account_deck::repository::account_deck_repository::AccountDeckRepository;
use crate::account_deck::repository::account_deck_repository_impl::AccountDeckRepositoryImpl;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::request::account_deck_list_request::AccountDeckListRequest;
use crate::account_deck::service::request::account_deck_modify_request::AccountDeckModifyRequest;
use crate::account_deck::service::request::account_deck_register_request::AccountDeckRegisterRequest;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;
use crate::account_deck::service::response::account_deck_modify_response::AccountDeckModifyResponse;
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
    async fn get_account_unique_id(&self, account_session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(account_session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }
}

#[async_trait]
impl AccountDeckService for AccountDeckServiceImpl {
    async fn account_deck_register(&self, account_deck_register_request: AccountDeckRegisterRequest) -> AccountDeckRegisterResponse {
        println!("AccountDeckServiceImpl: account_deck_register()");

        let account_unique_id = self.get_account_unique_id(account_deck_register_request.account_id()).await;
        let account_deck = AccountDeck::new(account_unique_id, account_deck_register_request.deck_name()).unwrap();

        let account_deck_repository_guard = self.repository.lock().await;

        let account_deck_count_limit = 6;
        if let Some(account_deck_list) =
            account_deck_repository_guard.get_list_by_user_int_id(account_unique_id).await.unwrap() {
            if account_deck_list.len() >= account_deck_count_limit {
                return AccountDeckRegisterResponse::new(false)
            }
        }

        let account_deck_save_result = account_deck_repository_guard.save(account_deck).await;

        drop(account_deck_repository_guard);

        if account_deck_save_result.is_err() { return AccountDeckRegisterResponse::new(false) }

        AccountDeckRegisterResponse::new(true)
    }

    async fn account_deck_list(&self, account_deck_list_request: AccountDeckListRequest) -> AccountDeckListResponse {
        println!("AccountDeckServiceImpl: account_deck_list()");

        let account_unique_id = self.get_account_unique_id(account_deck_list_request.account_id()).await;

        let account_deck_repository_guard = self.repository.lock().await;

        if let Some(deck_list) =
            account_deck_repository_guard.get_list_by_user_int_id(account_unique_id).await.unwrap() {
            return AccountDeckListResponse::new(deck_list)
        } else {
            let empty_set = Vec::new();
            AccountDeckListResponse::new(empty_set)
        }
    }

    async fn account_deck_modify(&self, account_deck_modify_request: AccountDeckModifyRequest) -> AccountDeckModifyResponse {
        println!("AccountDeckServiceImpl: account_deck_modify()");

        let account_unique_id = self.get_account_unique_id(account_deck_modify_request.account_id()).await;

        let account_deck_repository_guard = self.repository.lock().await;

        let result = account_deck_repository_guard.update_data(account_deck_modify_request, account_unique_id).await;

        if result.is_ok() {
            AccountDeckModifyResponse::new(true)
        } else {
            AccountDeckModifyResponse::new(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_deck_registration() {
        let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
        let mut account_deck_service_mutex_guard = account_deck_service_mutex.lock().await;

        let redis_token_str = "redis_token_str";
        let sample_deck_name = "7th deck";

        let account_deck_register_request = AccountDeckRegisterRequest::new(redis_token_str.to_string(), sample_deck_name.to_string());

        let result = account_deck_service_mutex_guard.account_deck_register(account_deck_register_request).await;

        assert_eq!(false, result.get_is_success());
    }

    #[test]
    async fn test_deck_list() {
        let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
        let mut account_deck_service_mutex_guard = account_deck_service_mutex.lock().await;
        let redis_token_str = "redis_token_str";
        let account_deck_list_request = AccountDeckListRequest::new(redis_token_str.to_string());
        let result = account_deck_service_mutex_guard.account_deck_list(account_deck_list_request).await;
        println!("{:?}", result.get_list())
    }

    #[test]
    async fn test_deck_modify() {
        let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
        let mut account_deck_service_mutex_guard = account_deck_service_mutex.lock().await;

        let sample_deck_id: i32 = 8;
        let sample_account_id_str = "redis_token_str";
        let sample_deck_name = "fantastic deck name";

        let account_deck_modify_request
            = AccountDeckModifyRequest::new(sample_deck_id, sample_account_id_str.to_string(), sample_deck_name.to_string());

        let result = account_deck_service_mutex_guard.account_deck_modify(account_deck_modify_request).await;

        assert_eq!(true, result.get_is_success())
    }

    #[tokio::test]
    #[cfg(not(feature = "account_deck_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}