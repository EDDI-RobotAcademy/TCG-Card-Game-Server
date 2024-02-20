use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::{Mutex as AsyncMutex, Mutex};

use crate::shop::controller::shop_controller::ShopController;
use crate::shop::controller::request_form::execute_shop_gacha_request_form::ExecuteShopGachaRequestForm;
use crate::shop::controller::request_form::execute_free_gacha_request_form::ExecuteFreeGachaRequestForm;
use crate::shop::controller::response_form::execute_shop_gacha_response_form::ExecuteShopGachaResponseForm;
use crate::shop::controller::response_form::execute_free_gacha_response_form::ExecuteFreeGachaResponseForm;


use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::account_point_service_impl::AccountPointServiceImpl;

use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

use crate::shop_gacha::service::shop_gacha_service::ShopGachaService;
use crate::shop_gacha::service::shop_gacha_service_impl::ShopGachaServiceImpl;

pub struct ShopControllerImpl {
    account_point_service: Arc<AsyncMutex<AccountPointServiceImpl>>,
    shop_gacha_service: Arc<AsyncMutex<ShopGachaServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
}

impl ShopControllerImpl {
    pub fn new(account_point_service: Arc<AsyncMutex<AccountPointServiceImpl>>,
               shop_gacha_service: Arc<AsyncMutex<ShopGachaServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    ) -> Self {
        ShopControllerImpl {
            account_point_service,
            shop_gacha_service,
            redis_in_memory_service,
        }
    }
    pub fn get_instance() -> Arc<Mutex<ShopControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopControllerImpl::new(
                            AccountPointServiceImpl::get_instance(),
                            ShopGachaServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // async fn get_account_unique_id(&self, session_id: &str) -> i32 {
    //     let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
    //     let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
    //     let account_unique_id_string = account_unique_id_option_string.unwrap();
    //     let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
    //     account_unique_id
    // }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl ShopController for ShopControllerImpl {
    async fn execute_shop_gacha(&self, execute_shop_gacha_request_form: ExecuteShopGachaRequestForm) -> ExecuteShopGachaResponseForm {
        let account_unique_id = self.is_valid_session(execute_shop_gacha_request_form.to_session_validation_request()).await;
        //1. 재화 확인
        //2. 재화 사용
        let account_point_service_guard = self.account_point_service.lock().await;
        let check_pay_gold_response = account_point_service_guard.pay_gold(
            execute_shop_gacha_request_form.to_pay_gole_request(account_unique_id, 100)
        ).await;
        if !check_pay_gold_response.get_is_success() {
            return ExecuteShopGachaResponseForm::new(vec![0], false);
        }
        //3. 카드 뽑기
        let shop_gacha_service_guard = self.shop_gacha_service.lock().await;
        let get_specific_race_card_response = shop_gacha_service_guard.get_specific_race_card_default(
            execute_shop_gacha_request_form.to_get_specific_race_card_request(
                account_unique_id,
                execute_shop_gacha_request_form.get_race_enum(),
                execute_shop_gacha_request_form.is_confirmed_upper_legend())).await;

        ExecuteShopGachaResponseForm::new(get_specific_race_card_response.get_card_id_list(), true)
    }
    async fn execute_free_gacha(&self, execute_free_gacha_request_form: ExecuteFreeGachaRequestForm) -> ExecuteFreeGachaResponseForm {
        //1. 무료 뽑기 가능한지 확인
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(
            execute_free_gacha_request_form.to_session_validation_request()).await;
        let value_string = session_validation_response.get_value();
        let save_daily_token_redis_response = redis_in_memory_service_guard.save_daily_key_and_value(
            execute_free_gacha_request_form.to_save_daily_key_and_value_request(
                session_validation_response.get_value())).await;

        if !save_daily_token_redis_response.is_success() {
            return ExecuteFreeGachaResponseForm::new(vec![0], false);
        }

        //2. 카드 뽑기
        let shop_gacha_service_guard = self.shop_gacha_service.lock().await;
        let get_specific_race_card_response = shop_gacha_service_guard.get_specific_race_card_default(
            execute_free_gacha_request_form.to_get_specific_race_card_request(
                value_string.parse::<i32>().unwrap_or_else(|_| { -1 }),
                execute_free_gacha_request_form.get_race_enum(),
                execute_free_gacha_request_form.is_confirmed_upper_legend())).await;

        ExecuteFreeGachaResponseForm::new(get_specific_race_card_response.get_card_id_list(), true)
    }

}