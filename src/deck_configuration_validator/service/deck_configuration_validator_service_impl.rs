use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account_card::repository::account_card_repository::AccountCardRepository;
use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;

use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::common::converter::vector_to_hash_converter::VectorToHashConverter;
use crate::deck_configuration_validator::repository::deck_configuration_validator_repository::DeckConfigurationValidatorRepository;
use crate::deck_configuration_validator::repository::deck_configuration_validator_repository_impl::DeckConfigurationValidatorRepositoryImpl;

use crate::deck_configuration_validator::service::deck_configuration_validator_service::DeckConfigurationValidatorService;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct DeckConfigurationValidatorServiceImpl {
    deck_configuration_repository: Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
    account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
}

impl DeckConfigurationValidatorServiceImpl {
    pub fn new(deck_configuration_repository: Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>,
               account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,


    ) -> Self {
        DeckConfigurationValidatorServiceImpl {
            deck_configuration_repository,
            card_grade_repository,
            account_card_repository,
            redis_in_memory_repository,
            card_kinds_repository,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        DeckConfigurationValidatorServiceImpl::new(
                            DeckConfigurationValidatorRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance(),
                            AccountCardRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance())));
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
impl DeckConfigurationValidatorService for DeckConfigurationValidatorServiceImpl {
    async fn validate_deck(&self, deck: &Vec<i32>) -> Result<(), String> {
        println!("DeckConfigurationValidatorServiceImpl: validate_deck()");

        let deck_configuration_repository_guard =
            self.deck_configuration_repository.lock().await;

        // 1. check deck card list count
        if let Err(counting_error_message) =
            deck_configuration_repository_guard.counter(deck).await {
            return Err(counting_error_message)
        }

        // 2. check duplication

        // get energy card list
        let card_kinds_repository_guard
            = self.card_kinds_repository.lock().await;
        let card_grade_repository_guard
            = self.card_grade_repository.lock().await;

        let mut energy_card_list = Vec::new();
        for card in deck {
            let kind_result = card_kinds_repository_guard.get_card_kind(card).await;
            let grade_result = card_grade_repository_guard.get_card_grade(card).await;

            if kind_result == KindsEnum::Energy {
                if grade_result == GradeEnum::Common {
                    energy_card_list.push(card);
                }
            }
        }

        if let Err(duplication_error_message) =
            deck_configuration_repository_guard.duplication_checker(deck, energy_card_list).await {
            return Err(duplication_error_message)
        }

        // 3. check grade limit

        // let mut normal_card_count = 0;
        let mut uncommon_card_count = 0;
        let mut hero_card_count = 0;
        let mut legendary_card_count = 0;
        let mut mythical_card_count = 0;

        // let normal_card_count_limit = 12;
        let uncommon_card_count_limit = 15;
        let hero_card_count_limit = 9;
        let legendary_card_count_limit = 3;
        let mythical_card_count_limit = 1;

        // TODO: 추가 리팩토링 필요함
        Ok(for card in deck {
            let grade_result = card_grade_repository_guard.get_card_grade(card).await;

            if grade_result == GradeEnum::Uncommon {
                uncommon_card_count += 1;
                if uncommon_card_count > uncommon_card_count_limit {
                    let card_count_error =
                        format!("{:?} 등급 카드 최대치: {}장 초과", grade_result, uncommon_card_count_limit);
                    return Err(card_count_error)
                }
            }

            if grade_result == GradeEnum::Hero {
                hero_card_count += 1;
                if hero_card_count > hero_card_count_limit {
                    let card_count_error =
                        format!("{:?} 등급 카드 최대치: {}장 초과", grade_result, hero_card_count_limit);
                    return Err(card_count_error)
                }
            }

            if grade_result == GradeEnum::Legend {
                legendary_card_count += 1;
                if legendary_card_count > legendary_card_count_limit {
                    let card_count_error =
                        format!("{:?} 등급 카드 최대치: {}장 초과", grade_result, legendary_card_count_limit);
                    return Err(card_count_error)
                }
            }

            if grade_result == GradeEnum::Mythical {
                mythical_card_count += 1;
                if mythical_card_count > mythical_card_count_limit {
                    let card_count_error =
                        format!("{:?} 등급 카드 최대치: {}장 초과", grade_result, mythical_card_count_limit);
                    return Err(card_count_error)
                }
            }
        })
    }
    async fn do_you_have_this_card(&self, card_list: Vec<i32>, account_session_id: &str) -> bool {
        let account_unique_id = self.get_account_unique_id(account_session_id).await;

        let card_list_vec = VectorToHashConverter::convert_vector_to_hash(&card_list);
        //1. 해당 어카운트의 카드 가져오기
        let account_card_repository_guard = self.accuont_card_repository.lock().await;
        let account_card_list = account_card_repository_guard.get_card_list(account_unique_id).await.unwrap().unwrap();
        //2. 어카운트의 카드 리스트와 받아온 카드리스트 비교
        for (card_id, card_count) in card_list_vec {
            let mut check_contain_key = false;
            for account_card in &account_card_list {
                if account_card.contains_key(&card_id) {
                    check_contain_key = true;
                    if account_card[&card_id] < card_count { return false; }
                }
            }
            if check_contain_key == false { return false; }
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_deck_validation() {
        let deck_configuration_validator_service
            = DeckConfigurationValidatorServiceImpl::get_instance();
        let deck_configuration_validator_service_guard
            = deck_configuration_validator_service.lock().await;

        let sample_deck = [19, 8, 8, 8, 9, 25, 25, 25, 27, 100,
                                    27, 27, 151, 20, 20, 20, 2, 2, 2, 26,
                                    26, 26, 30, 31, 31, 31, 32, 32, 32, 33,
                                    33, 35, 93, 93, 93, 93, 93, 93, 93, 93].to_vec();

        let validation_result = deck_configuration_validator_service_guard.validate_deck(&sample_deck).await;
        if validation_result.is_ok() {
            assert!(true)
        }
        if validation_result.is_err() {
            println!("{:?}", validation_result.unwrap_err());
        }
    }
    #[test]
    async fn test_do_you_have() {
        let deck_configuration_validator_service
            = DeckConfigurationValidatorServiceImpl::get_instance();
        let deck_configuration_validator_service_guard
            = deck_configuration_validator_service.lock().await;

        let sample_card_list = [30,3,4,3,3].to_vec();

        let validation_result = deck_configuration_validator_service_guard.do_you_have_this_card(sample_card_list , "qwer").await;

        println!("{}", validation_result);
    }
}