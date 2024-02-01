use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::verify;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account::service::request::account_delete_request::AccountDeleteRequest;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account_card::service::request::account_card_list_request::AccountCardListRequest;
use crate::account_card::service::response::account_card_list_response::AccountCardListResponse;
use crate::account_point::entity::account_point::account_points::account_id;
use crate::account_point::repository::account_point_repository::AccountPointRepository;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;

use crate::account_point::repository::account_point_repository_impl::AccountPointRepositoryImpl;
use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::request::gain_gold_request::GainGoldRequest;
use crate::account_point::service::response::gain_gold_response::GainGoldResponse;

use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct AccountPointServiceImpl {
    repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
    account_repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl AccountPointServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
               account_repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        AccountPointServiceImpl {
            repository,
            account_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountPointServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountPointServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountPointServiceImpl::new(
                            AccountPointRepositoryImpl::get_instance(),
                            AccountRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountPointService for AccountPointServiceImpl {
    async fn gain_gold(&self, gain_gold_request: GainGoldRequest ) -> GainGoldResponse {
        println!("AccountPointServiceImpl: pay_gold()");

        let account_point_repository = self.repository.lock().await;
        let account_repository = self.account_repository.lock().await;
        let current_account_id: Result<i32, _> = gain_gold_request.account_id().parse();
        let current_gold: Result<i32, _>= gain_gold_request.gold().parse();
        let gain_gold: Result<i32, _> = gain_gold_request.gold().parse();
        let current_account_id_int: i32 = current_account_id.unwrap();
        let current_gold_int: i32 = current_gold.unwrap();
        let gain_gold_int: i32 = gain_gold.unwrap();
        let result_gold = current_gold_int + gain_gold_int;

        if let Some(found_account) = account_point_repository.find_by_account_id(current_account_id_int).await.unwrap() {

            let result = account_point_repository.update_gold(found_account, result_gold).await;
            if result.is_ok() {
                return GainGoldResponse::new(true)
            }
            return GainGoldResponse::new(false)
        }
        return GainGoldResponse::new(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_gain_gold() {
        let account_point_service_mutex = AccountPointServiceImpl::get_instance();
        let account_point_service_mutex_guard = account_point_service_mutex.lock().await;

        let gain_gold_request = GainGoldRequest::new(2, 200);

        let result = account_point_service_mutex_guard.gain_gold(gain_gold_request).await;
        println!("{:?}", result);
    }

    // #[tokio::test]
    // #[cfg(not(feature = "account_card_test"))]
    // async fn dummy_test() {
    //     assert!(true);
    // }
}