use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

use crate::account_card::repository::account_card_repository::AccountCardRepository;
use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;

use crate::card_race::repository::card_race_repository::CardRaceRepository;
use crate::card_race::repository::card_race_repository_impl::CardRaceRepositoryImpl;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

use crate::shop::repository::shop_repository::ShopRepository;
use crate::shop::repository::shop_repository_impl::ShopRepositoryImpl;

use crate::shop::service::request::free_card_request::FreeCardRequest;
use crate::shop::service::response::free_card_response::FreeCardResponse;

use crate::shop::service::request::get_card_default_request::GetCardDefaultRequest;
use crate::shop::service::response::get_card_default_response::GetCardDefaultResponse;

use crate::shop::service::shop_service::ShopService;

pub struct ShopServiceImpl {
    repository: Arc<AsyncMutex<ShopRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>
}

impl ShopServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<ShopRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>) -> Self {
        ShopServiceImpl {
            repository,
            redis_in_memory_repository,
            account_card_repository,
            card_race_repository,
            card_grade_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopServiceImpl::new(
                            ShopRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            AccountCardRepositoryImpl::get_instance(),
                            CardRaceRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance())));
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
    async fn card_gacha_system (&self, grade_card_list: HashMap<i32,GradeEnum>, how_many_cards_to_get: i32) -> Vec<i32> {
        let shop_repository = self.repository.lock().await;
        let gacha_grade_result = shop_repository.apply_probability_by_grade(how_many_cards_to_get, true);

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
impl ShopService for ShopServiceImpl {

    async fn get_specific_race_card_default(&self, get_card_default_request: GetCardDefaultRequest) -> GetCardDefaultResponse {

        let card_race_repository = self.card_race_repository.lock().await;
        let card_grade_repository = self.card_grade_repository.lock().await;

        let account_unique_id = self.get_account_unique_id(get_card_default_request.account_id()).await;

        // 뽑을 카드 리스트
        let specific_race_card_list = card_race_repository.get_specific_race_card_list(get_card_default_request.get_race_enum()).await;
        let gacha_card_list = card_grade_repository.get_grade_by_card_list(specific_race_card_list).await;
        // 카드 10개 뽑기
        let get_card_list = self.card_gacha_system(gacha_card_list, 10).await;

        self.update_account_card_db(account_unique_id, get_card_list.clone()).await;

        GetCardDefaultResponse::new(get_card_list)
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

        let request = GetCardDefaultRequest::new("qwer".to_string(), "Human".to_string());

        let result = shop_service_impl_mutex_guard.get_specific_race_card_default(request).await;

        println!("result: {:?}", result);
    }
}