use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;

use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::deck_configuration_validator::repository::deck_configuration_validator_repository::DeckConfigurationValidatorRepository;
use crate::deck_configuration_validator::repository::deck_configuration_validator_repository_impl::DeckConfigurationValidatorRepositoryImpl;

use crate::deck_configuration_validator::service::deck_configuration_validator_service::DeckConfigurationValidatorService;

pub struct DeckConfigurationValidatorServiceImpl {
    deck_configuration_repository: Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>>,
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>
}

impl DeckConfigurationValidatorServiceImpl {
    pub fn new(deck_configuration_repository: Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>>,
               card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>) -> Self {
        DeckConfigurationValidatorServiceImpl {
            deck_configuration_repository,
            card_grade_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        DeckConfigurationValidatorServiceImpl::new(
                            DeckConfigurationValidatorRepositoryImpl::get_instance(),
                            CardGradeRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
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
        if let Err(duplication_error_message) =
            deck_configuration_repository_guard.duplication_checker(deck).await {
            return Err(duplication_error_message)
        }

        // 3. check grade limit
        let card_grade_repository_guard
            = self.card_grade_repository.lock().await;

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
                                    33, 35, 35, 36, 36, 93, 93, 93, 100, 100].to_vec();

        let validation_result = deck_configuration_validator_service_guard.validate_deck(&sample_deck).await;
        if validation_result.is_ok() {
            assert!(true)
        }
        if validation_result.is_err() {
            println!("{:?}", validation_result.unwrap_err());
        }
    }
}