use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::deck_configuration_validator::repository::deck_configuration_validator_repository::DeckConfigurationValidatorRepository;

pub struct DeckConfigurationValidatorRepositoryImpl {}

impl DeckConfigurationValidatorRepositoryImpl {
    pub fn new() -> Self { DeckConfigurationValidatorRepositoryImpl {} }
    pub fn get_instance() -> Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckConfigurationValidatorRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        DeckConfigurationValidatorRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl DeckConfigurationValidatorRepository for DeckConfigurationValidatorRepositoryImpl {
    async fn counter(&self, deck: &Vec<i32>) -> Result<(), String> {
        match deck.len() {
            40 => {
                Ok(())
            }
            length => {
                // 40장이 아닌 경우
                let error_string = format!("덱에 총 {}장이 있습니다. 정확히 40장을 맞춰주세요!", length);

                return Err(error_string);
            }
        }
    }
    async fn duplication_checker(&self, deck: &Vec<i32>, energy_card_list: Vec<&i32>) -> Result<(), String> {
        let mut card_count_map = HashMap::new();

        for card_id in deck.iter() {
            let card_counts = card_count_map.entry(card_id).or_insert(0);
            *card_counts += 1;

            if *card_counts > 3 {
                if !energy_card_list.iter().find(|x| x == &&card_id).is_some() {
                    let error_string =
                        format!("{}번 카드가 3장이 넘습니다. 같은 카드는 덱에 3장 이하여야 합니다!", card_id);

                    return Err(error_string);
                }

            }
        }
        Ok(())
    }
}