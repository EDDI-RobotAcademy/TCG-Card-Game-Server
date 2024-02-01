use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::MutexGuard as AsyncMutexGuard;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;

pub struct CardKindsRepositoryImpl {
    card_kinds_map: Arc<AsyncMutex<HashMap<String, String>>>,
}

impl CardKindsRepositoryImpl {
    pub fn new() -> Self {
        CardKindsRepositoryImpl {
            card_kinds_map: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }

    pub async fn get_card_kinds_map(&self) -> AsyncMutexGuard<'_, HashMap<String, String>> {
        self.card_kinds_map.lock().await
    }

    pub async fn set_card_kinds_map(&self, card_kinds_dictionary: HashMap<String, String>) {
        let mut card_kinds_map_guard = self.card_kinds_map.lock().await;
        *card_kinds_map_guard = card_kinds_dictionary;
    }

    pub async fn create_instance(card_kinds_dictionary: HashMap<String, String>) {
        let instance = Self::get_instance();
        let mut instance_guard = instance.lock().await;
        instance_guard.set_card_kinds_map(card_kinds_dictionary).await;
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
    async fn get_card_kind(&self, card_number: &str) -> Option<String> {
        let card_kinds_map_guard = self.card_kinds_map.lock().await;
        card_kinds_map_guard.get(card_number).cloned()
    }
}

