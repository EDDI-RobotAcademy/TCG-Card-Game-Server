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
}

#[async_trait]
impl ShopService for ShopServiceImpl {
    // async fn free_card(&self, free_card_request: FreeCardRequest) -> FreeCardResponse {
    //     println!("ShopServiceImpl: free_card()");
    //
    //     let shop_repository = self.repository.lock().await;
    //     let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
    //     let account_number_str = redis_repository_guard.get(free_card_request.account_id()).await;
    //     let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();
    //     match account_unique_id {
    //         Ok(int_type_account_id) => {
    //             let result = shop_repository.add_free_cards(int_type_account_id).await;
    //             if result.is_ok() {
    //                 FreeCardResponse::new(result.unwrap())
    //             } else {
    //                 let empty_set = Vec::new();
    //                 FreeCardResponse::new(empty_set)
    //             }
    //         }
    //         Err(e) => {
    //             let empty_set = Vec::new();
    //             FreeCardResponse::new(empty_set)
    //         }
    //     }
    // }
    async fn get_card_default(&self, get_card_default_request: GetCardDefaultRequest) -> GetCardDefaultResponse {

        let shop_repository = self.repository.lock().await;
        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_card_repository = self.account_card_repository.lock().await;
        let card_race_repository = self.card_race_repository.lock().await;
        let card_grade_repository = self.card_grade_repository.lock().await;

        let account_number_str = redis_repository_guard.get(get_card_default_request.account_id()).await;
        let account_unique_id: Result<i32, _> = account_number_str.expect("REASON").parse();

        let get_race_specific_card_list = card_race_repository.get_specific_race_card_list(get_card_default_request.get_race_enum()).await;
        let gacha_card_list = card_grade_repository.get_grade_by_specific_race_card_list(get_race_specific_card_list).await;

        match account_unique_id {
            Ok(int_type_account_id) => {
                // 카드 10개 뽑기
                let get_cards = shop_repository.get_randomly_chosen_card_id_list(10, gacha_card_list).await.unwrap();
                // 뽑은 사용자의 카드 리스트 불러오기
                let get_account_card_list = account_card_repository.get_card_list(int_type_account_id).await.unwrap().unwrap();
                // 뽑은 카드와 사용자의 카드 리스트 비교
                let account_card_check = account_card_repository.check_same_card(get_cards.clone(), get_account_card_list).await;

                for checked_card in account_card_check {
                    if (checked_card.1 != 0){
                        account_card_repository.update_card_count(int_type_account_id, checked_card).await;
                    }
                    if (checked_card.1 == 0){
                        account_card_repository.save_new_card(int_type_account_id, checked_card.0).await;
                    }
                }
                GetCardDefaultResponse::new(get_cards)
                }

            Err(e) => {
                let empty_set = Vec::new();
                GetCardDefaultResponse::new(empty_set)
            }

        }

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

        let request = GetCardDefaultRequest::new("qwer".to_string(), "전체".to_string());

        let result = shop_service_impl_mutex_guard.get_card_default(request).await;

        println!("result: {:?}", result);
    }
}