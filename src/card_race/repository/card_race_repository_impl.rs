use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_race::repository::card_race_repository::CardRaceRepository;
use crate::common::csv::csv_reader::{build_card_race_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardRaceRepositoryImpl {
    card_race_map: Arc<AsyncMutex<HashMap<i32, String>>>,
}

impl CardRaceRepositoryImpl {
    pub fn new() -> Self {
        let filename = RootPath::make_full_path("resources/csv/every_card.csv")
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
    async fn get_card_race(&self, card_number: &i32) -> Option<String> {
        let card_race_map_guard = self.card_race_map.lock().await;
        card_race_map_guard.get(card_number).cloned()
    }

    async fn get_race_specific_card_list(&self , race_name : &str) -> Vec<i32> {
        let mut race_specific_card_list = Vec::new();
        let card_race_map_guard = self.card_race_map.lock().await;

        for card in card_race_map_guard.clone() {
            if(card.1 == race_name) {
                race_specific_card_list.push(card.0);
            }
            if(card.1 == "전체") {
                race_specific_card_list.push(card.0);
            }
        }
        race_specific_card_list


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

        match card_race {
            Some(race) => {
                println!("Card Grade: {}", race);
                assert_eq!(race, "휴먼");
            }
            None => println!("Card not found."),
        }
    }
}