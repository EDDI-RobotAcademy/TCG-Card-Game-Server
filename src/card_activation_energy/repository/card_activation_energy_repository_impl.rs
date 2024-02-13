use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::card_activation_energy::repository::card_activation_energy_repository::CardActivationEnergyRepository;
use crate::common::card_attributes::card_activation_energy::activation_energy_enum::ActivationEnergyEnum;
use crate::common::csv::csv_reader::{build_card_activation_energy_dictionary, csv_read};
use crate::common::path::root_path::RootPath;


pub struct CardActivationEnergyRepositoryImpl {
    card_activation_energy_map: Arc<AsyncMutex<HashMap<i32, i32>>>,
}

impl CardActivationEnergyRepositoryImpl {
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

        let card_activation_energy_dictionary = build_card_activation_energy_dictionary(&csv_content);

        CardActivationEnergyRepositoryImpl {
            card_activation_energy_map: Arc::new(AsyncMutex::new(card_activation_energy_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardActivationEnergyRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardActivationEnergyRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardActivationEnergyRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardActivationEnergyRepository for CardActivationEnergyRepositoryImpl {
    async fn get_card_activation_energy(&self, card_number: &i32) -> i32 {
        let card_card_activation_energy_map_guard = self.card_activation_energy_map.lock().await;
        card_card_activation_energy_map_guard.get(card_number).unwrap_or(&0).clone()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_activation_energy() {
        let card_activation_energy_repository_mutex = CardActivationEnergyRepositoryImpl::get_instance();
        let card_activation_energy_repository_guard = card_activation_energy_repository_mutex.lock().await;
        let card_number: i32 = 6;
        let card_activation_energy = card_activation_energy_repository_guard.get_card_activation_energy(&card_number).await;



        println!("card_activation_energy: {:?}", card_activation_energy);
    }
}


