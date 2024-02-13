use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::card_attack_point::repositoty::card_attack_point_repository::CardAttackPointRepository;
use crate::common::card_attributes::card_attack_point::attack_point_enum::AttackPointEnum;
use crate::common::csv::csv_reader::{build_card_attack_point_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardAttackPointRepositoryImpl {
    card_attack_point_map: Arc<AsyncMutex<HashMap<i32, i32>>>,
}

impl CardAttackPointRepositoryImpl {
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

        let card_attack_point_dictionary = build_card_attack_point_dictionary(&csv_content);

        CardAttackPointRepositoryImpl {
            card_attack_point_map: Arc::new(AsyncMutex::new(card_attack_point_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardAttackPointRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardAttackPointRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardAttackPointRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardAttackPointRepository for CardAttackPointRepositoryImpl {
    async fn get_card_attack_point(&self, card_number: &i32) -> i32 {
        let card_attack_point_map_guard = self.card_attack_point_map.lock().await;
        card_attack_point_map_guard.get(card_number).unwrap_or(&0).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_get_card_attack_point() {
        let card_attack_point_repository_mutex = CardAttackPointRepositoryImpl::get_instance();
        let card_attack_point_repository_guard = card_attack_point_repository_mutex.lock().await;
        let card_number: i32 = 6;
        let card_attack_point = card_attack_point_repository_guard.get_card_attack_point(&card_number).await;



        println!("card_attack_point: {:?}", card_attack_point);
    }
}


