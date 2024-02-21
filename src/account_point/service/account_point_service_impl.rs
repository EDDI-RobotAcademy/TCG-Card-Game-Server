use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account_point::repository::account_point_repository::AccountPointRepository;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;
use crate::account_point::entity::account_point::AccountPoint;

use crate::account_point::repository::account_point_repository_impl::AccountPointRepositoryImpl;
use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::request::gain_gold_request::GainGoldRequest;
use crate::account_point::service::request::pay_gold_request::PayGoldRequest;
use crate::account_point::service::response::gain_gold_response::GainGoldResponse;
use crate::account_point::service::response::pay_gold_response::PayGoldResponse;

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
        let current_account_id: Result<i32, _> = gain_gold_request.account_id().parse();
        let current_account_id_int: i32 = current_account_id.unwrap();

        if let Some(found_account) = account_point_repository.find_by_account_id(current_account_id_int).await.unwrap() {
            let gain_gold: Result<i32, _> = gain_gold_request.gold().parse();
            let current_gold_int: i32 = found_account.gold();
            let gain_gold_int: i32 = gain_gold.unwrap();
            let result_gold = current_gold_int + gain_gold_int;
            let result = account_point_repository.update_gold(found_account, result_gold).await;
            if result.is_ok() {
                return GainGoldResponse::new(true)
            }
            return GainGoldResponse::new(false)
        }
        return GainGoldResponse::new(false)
    }

    async fn pay_gold(&self, pay_gold_request: PayGoldRequest ) -> PayGoldResponse {
        println!("AccountPointServiceImpl: pay_gold()");

        let account_point_repository = self.repository.lock().await;
        let current_account_id: Result<i32, _> = pay_gold_request.account_id().parse();
        let current_account_id_int: i32 = current_account_id.unwrap();


        if let Some(found_account) = account_point_repository.find_by_account_id(current_account_id_int).await.unwrap() {
            let pay_gold: Result<i32, _> = pay_gold_request.gold().parse();
            let current_gold_int: i32 = found_account.gold();
            let pay_gold_int: i32 = pay_gold.unwrap();
            let result_gold = current_gold_int - pay_gold_int;
            if result_gold < 0 {
                println!("You don't have enough gold.");
                return PayGoldResponse::new(false)
            }
            let result = account_point_repository.update_gold(found_account, result_gold).await;
            if result.is_ok() {
                return PayGoldResponse::new(true)
            }
            return PayGoldResponse::new(false)
        }
        return PayGoldResponse::new(false)
    }
    async fn find_by_account_id(&self, account_unique_id: i32 ) -> AccountPoint {
        let account_point_repository = self.repository.lock().await;
        let account_point = account_point_repository.find_by_account_id(account_unique_id).await.unwrap();

        account_point.unwrap()
    }
    async fn update_event_check(&self, account_unique_id: i32) {
        let account_point_repository = self.repository.lock().await;
        account_point_repository.update_event_check(account_unique_id).await.expect("TODO: panic message");
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

        let gain_gold_request = GainGoldRequest::new(3, 1000);

        let result = account_point_service_mutex_guard.gain_gold(gain_gold_request).await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_pay_gold() {
        let account_point_service_mutex = AccountPointServiceImpl::get_instance();
        let account_point_service_mutex_guard = account_point_service_mutex.lock().await;

        let pay_gold_request = PayGoldRequest::new(1, 200);

        let result = account_point_service_mutex_guard.pay_gold(pay_gold_request).await;
        println!("{:?}", result);
    }
}