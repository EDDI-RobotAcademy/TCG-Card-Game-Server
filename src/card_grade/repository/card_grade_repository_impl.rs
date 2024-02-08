use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::common::csv::csv_reader::{build_card_grade_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardGradeRepositoryImpl {
    card_grade_map: Arc<AsyncMutex<HashMap<i32, GradeEnum>>>,
}

impl CardGradeRepositoryImpl {
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

        let card_grade_dictionary = build_card_grade_dictionary(&csv_content);

        CardGradeRepositoryImpl {
            card_grade_map: Arc::new(AsyncMutex::new(card_grade_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardGradeRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardGradeRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardGradeRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardGradeRepository for CardGradeRepositoryImpl {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum {
        let card_grade_map_guard = self.card_grade_map.lock().await;
        *card_grade_map_guard.get(card_number).unwrap_or(&GradeEnum::Dummy)
    }

    async fn get_grade_by_specific_race_card_list(&self, race_specific_card_id_list: Vec<i32>) -> Vec<(i32, GradeEnum)> {
        let card_grade_map_guard = self.card_grade_map.lock().await;
        let mut card_grade_list_by_race : Vec<(i32, GradeEnum)> = Vec::new();

        for card_grade in card_grade_map_guard.clone() {
            for race_card in race_specific_card_id_list.clone() {
                if (card_grade.0 == race_card) {
                    let card_tuple = (race_card, card_grade.1.clone());
                    card_grade_list_by_race.push(card_tuple);
                }
            }
        }

        card_grade_list_by_race
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_grade() {
        let card_grade_repository_mutex = CardGradeRepositoryImpl::get_instance();
        let card_grade_repository_guard = card_grade_repository_mutex.lock().await;
        let card_number: i32 = 6;
        let card_grade = card_grade_repository_guard.get_card_grade(&card_number).await;

        println!("Card Grade: {:?}", card_grade);
        assert_eq!(card_grade, GradeEnum::Common);
    }
}