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

    async fn account_deck_list(&self, account_deck_list_request: AccountDeckListRequest) -> AccountDeckListResponse {
        println!("AccountDeckServiceImpl: account_deck_list()");

        let account_deck_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_number_str = redis_repository_guard.get(account_deck_list_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
        match account_unique_id {
            Ok(int_type_account_id) => {
                if let Some(deck_list) = account_deck_repository.get_list_by_user_int_id(int_type_account_id).await.unwrap() {
                    AccountDeckListResponse::new(deck_list)
                } else {
                    let empty_set = Vec::new();
                    AccountDeckListResponse::new(empty_set)
                }
            }
            Err(e) => {
                println!("Deck list loading error : {}", e);
                let empty_set = Vec::new();
                AccountDeckListResponse::new(empty_set)
            }
        }
    }

    async fn account_deck_modify(&self, account_deck_modify_request: AccountDeckModifyRequest) -> AccountDeckModifyResponse {
        println!("AccountDeckServiceImpl: account_deck_modify()");

        let account_deck_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_number_str = redis_repository_guard.get(account_deck_modify_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
        match account_unique_id {
            Ok(int_id) => {
                let result = account_deck_repository.update_data(account_deck_modify_request, int_id).await;
                if result.is_ok() {
                    AccountDeckModifyResponse::new(true)
                } else {
                    AccountDeckModifyResponse::new(false)
                }
            }
            Err(e) => {
                eprintln!("Error to get int id : {}", e);
                AccountDeckModifyResponse::new(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

    #[test]
    async fn test_deck_registration() {
        let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
        let mut account_deck_service_mutex_guard = account_deck_service_mutex.lock().await;

        let redis_token_str = "redis_token_str";
        let sample_deck_name = "휴먼덱 화이팅";

        let account_deck_register_request = AccountDeckRegisterRequest::new(redis_token_str.to_string(), sample_deck_name.to_string());

        let result = account_deck_service_mutex_guard.account_deck_register(account_deck_register_request).await;

        assert_eq!(true, result.get_is_success());
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
        let sample_deck_name = "new deck name";

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