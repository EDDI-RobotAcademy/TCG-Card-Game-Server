use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_race::repository::card_race_repository::CardRaceRepository;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::common::csv::csv_reader::{build_card_race_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardRaceRepositoryImpl {
    card_race_map: Arc<AsyncMutex<HashMap<i32, RaceEnum>>>,
}

impl CardRaceRepositoryImpl {
    pub fn new() -> Self {
        let filename = RootPath::make_full_path("resources/csv/card_data.csv")
            .unwrap_or_else(|| {
                eprintln!("Failed to get file path.");
                std::process::exit(1);
            });
        let filename_path = &filename.to_string_lossy();

        let csv_content = match csv_read(filename_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading CSV file: {}", err);
                std::process::exit(1);
            }
        };

        let card_grade_dictionary = build_card_race_dictionary(&csv_content);

        CardRaceRepositoryImpl {
            card_race_map: Arc::new(AsyncMutex::new(card_grade_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardRaceRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardRaceRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardRaceRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardRaceRepository for CardRaceRepositoryImpl {
    async fn get_card_race(&self, card_number: &i32) -> RaceEnum {
        let card_race_map_guard = self.card_race_map.lock().await;
        *card_race_map_guard.get(card_number).unwrap_or(&RaceEnum::Dummy)
    }

    async fn get_specific_race_card_list(&self , race_value: RaceEnum) -> Vec<i32> {
        let mut specific_race_card_list = Vec::new();
        let card_race_map_guard = self.card_race_map.lock().await;

        for card in card_race_map_guard.clone() {
            if(card.1 == race_value) {
                specific_race_card_list.push(card.0);
            }
        }
        specific_race_card_list


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_race() {
        let card_race_repository_mutex = CardRaceRepositoryImpl::get_instance();
        let card_race_repository_guard = card_race_repository_mutex.lock().await;
        let card_number: i32 = 6;
        let card_race = card_race_repository_guard.get_card_race(&card_number).await;

        println!("Card Grade: {:?}", card_race);
        assert_eq!(card_race, RaceEnum::Undead);
    }
}