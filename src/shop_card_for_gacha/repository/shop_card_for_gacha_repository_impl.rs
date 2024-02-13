use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::shop_card_for_gacha::repository::shop_card_for_gacha_repository::ShopCardForGachaRepository;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::common::csv::csv_reader::{build_card_race_dictionary, build_shop_card_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct ShopCardForGachaRepositoryImpl {
    card_race_map: Arc<AsyncMutex<HashMap<i32, (RaceEnum, GradeEnum)>>>,
}

impl ShopCardForGachaRepositoryImpl {
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

        let shop_card_dictionary = build_shop_card_dictionary(&csv_content);

        ShopCardForGachaRepositoryImpl {
            card_race_map: Arc::new(AsyncMutex::new(shop_card_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ShopCardForGachaRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopCardForGachaRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopCardForGachaRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ShopCardForGachaRepository for ShopCardForGachaRepositoryImpl {
    async fn get_specific_race_card_list(&self , race_value: RaceEnum) -> HashMap<i32, GradeEnum> {
        let mut specific_race_card_list = HashMap::new();
        let card_race_map_guard = self.card_race_map.lock().await;

        for card in card_race_map_guard.clone() {
            if(card.1.0 == race_value) {
                specific_race_card_list.insert(card.0, card.1.1);
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
        // let card_race_repository_mutex = ShopCardForGachaRepositoryImpl::get_instance();
        // let card_race_repository_guard = card_race_repository_mutex.lock().await;
    }
}