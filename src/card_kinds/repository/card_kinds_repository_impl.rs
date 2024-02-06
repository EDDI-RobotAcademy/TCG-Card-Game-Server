use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::common::csv::csv_reader::{build_card_kinds_dictionary, csv_read};
use crate::common::path::root_path::RootPath;

pub struct CardKindsRepositoryImpl {
    card_kinds_map: Arc<AsyncMutex<HashMap<i32, i32>>>,
}

impl CardKindsRepositoryImpl {
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

        let card_kinds_dictionary = build_card_kinds_dictionary(&csv_content);

        CardKindsRepositoryImpl {
            card_kinds_map: Arc::new(AsyncMutex::new(card_kinds_dictionary)),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<CardKindsRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardKindsRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardKindsRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardKindsRepository for CardKindsRepositoryImpl {
    async fn get_card_kind(&self, card_number: &i32) -> Option<i32> {
        let card_kinds_map_guard = self.card_kinds_map.lock().await;
        card_kinds_map_guard.get(card_number).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_kind() {
        let card_kinds_repository_mutex = CardKindsRepositoryImpl::get_instance();
        let card_kinds_repository_guard = card_kinds_repository_mutex.lock().await;
        let card_number: i32 = 2;
        let card_kind = card_kinds_repository_guard.get_card_kind(&card_number).await;

        match card_kind {
            Some(kind) => {
                println!("Card Kind: {}", kind);
                assert_eq!(kind, 4);
            }
            None => println!("Card not found."),
        }
    }
}


