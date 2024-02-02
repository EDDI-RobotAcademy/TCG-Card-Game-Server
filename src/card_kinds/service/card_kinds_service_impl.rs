use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::card_kinds::service::card_kinds_service::CardKindsService;

pub struct CardKindsServiceImpl {
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>
}

impl CardKindsServiceImpl {
    pub fn new(card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>) -> Self {
        CardKindsServiceImpl {
            card_kinds_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardKindsServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardKindsServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardKindsServiceImpl::new(
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardKindsService for CardKindsServiceImpl {
    async fn get_card_kind(&self, card_number: &i32) -> Option<String> {
        println!("CardKindsServiceImpl: get_card_kind()");

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        card_kinds_repository_guard.get_card_kind(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_get_card_kind() {
        let card_kinds_repo_mutex = CardKindsServiceImpl::get_instance();
        let card_kinds_repo_guard = card_kinds_repo_mutex.lock().await;
        let card_number: i32 = 2;

        let card_kind = card_kinds_repo_guard.get_card_kind(&card_number).await;

        match card_kind {
            Some(kind) => {
                println!("Card Kind: {}", kind);
                assert_eq!(kind, "서포트");
            }
            None => println!("Card not found."),
        }
    }
}

