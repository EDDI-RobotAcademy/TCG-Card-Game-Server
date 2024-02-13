use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

use crate::account_card::repository::account_card_repository::AccountCardRepository;
use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;

use crate::shop_card_for_gacha::repository::shop_card_for_gacha_repository::ShopCardForGachaRepository;
use crate::shop_card_for_gacha::repository::shop_card_for_gacha_repository_impl::ShopCardForGachaRepositoryImpl;

use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

use crate::shop_gacha::repository::shop_gacha_repository::ShopGachaRepository;
use crate::shop_gacha::repository::shop_gacha_repository_impl::ShopGachaRepositoryImpl;

use crate::shop_gacha::service::request::get_specific_race_card_request::GetSpecificRaceCardRequest;
use crate::shop_gacha::service::response::get_specific_race_card_response::GetSpecificRaceCardResponse;

use crate::shop_gacha::service::shop_gacha_service::ShopGachaService;

pub struct ShopGachaServiceImpl {
    repository: Arc<AsyncMutex<ShopGachaRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    shop_card_for_gacha_repository: Arc<AsyncMutex<ShopCardForGachaRepositoryImpl>>
}

impl ShopGachaServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<ShopGachaRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               shop_card_for_gacha_repository: Arc<AsyncMutex<ShopCardForGachaRepositoryImpl>>) -> Self {
        ShopGachaServiceImpl {
            repository,
            redis_in_memory_repository,
            account_card_repository,
            shop_card_for_gacha_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopGachaServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopGachaServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopGachaServiceImpl::new(
                            ShopGachaRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            AccountCardRepositoryImpl::get_instance(),
                            ShopCardForGachaRepositoryImpl::get_instance())));
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
    async fn update_account_card_db(&self, account_unique_id: i32, update_card_list: Vec<i32>) {
        let account_card_repository = self.account_card_repository.lock().await;
        let get_account_card_list = account_card_repository.get_card_list(account_unique_id).await.unwrap().unwrap();
        let account_card_check = account_card_repository.check_same_card(update_card_list.clone(), get_account_card_list).await;

        for checked_card in account_card_check {
            if (checked_card.1 != 0){
                account_card_repository.update_card_count(account_unique_id, checked_card).await;
            }
            if (checked_card.1 == 0){
                account_card_repository.save_new_card(account_unique_id, checked_card.0).await;
            }
        }
    }
    async fn card_gacha_system (&self, grade_card_list: HashMap<i32,GradeEnum>, how_many_cards_to_get: i32, is_confirmed_upper_legend: bool) -> Vec<i32> {
        let shop_repository = self.repository.lock().await;
        let gacha_grade_result = shop_repository.apply_probability_by_grade(how_many_cards_to_get, is_confirmed_upper_legend);

        let mut get_card_list = Vec::new();
        for grade in gacha_grade_result {
            let card_list: Vec<_> = grade_card_list.clone().into_iter().filter(|s| s.1 == grade).collect();
            let get_cards = shop_repository.get_randomly_chosen_card_id(card_list).await;
            get_card_list.push(get_cards);
        }
        get_card_list
    }
}

#[async_trait]
impl ShopGachaService for ShopGachaServiceImpl {

    async fn get_specific_race_card_default(&self, get_specific_race_card_request: GetSpecificRaceCardRequest) -> GetSpecificRaceCardResponse {

        let shop_card_for_gacha_repository = self.shop_card_for_gacha_repository.lock().await;

        let account_unique_id = self.get_account_unique_id(get_specific_race_card_request.account_id()).await;
        // 뽑을 카드 리스트
        let specific_race_card_list = shop_card_for_gacha_repository.get_specific_race_card_list(get_specific_race_card_request.get_race_enum()).await;
        // 카드 10개 뽑기
        let get_card_list = self.card_gacha_system(specific_race_card_list, 10, get_specific_race_card_request.is_confirmed_upper_legend()).await;

        self.update_account_card_db(account_unique_id, get_card_list.clone()).await;

        GetSpecificRaceCardResponse::new(get_card_list)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_add_free_cards() {
        let shop_service_impl_mutex = ShopGachaServiceImpl::get_instance();
        let shop_service_impl_mutex_guard = shop_service_impl_mutex.lock().await;

        let request = GetSpecificRaceCardRequest::new("qwer".to_string(), "Human".to_string(), true);

        let result = shop_service_impl_mutex_guard.get_specific_race_card_default(request).await;

        println!("result: {:?}", result);
    }
}