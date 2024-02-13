use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_health_point::repository::card_health_point_repository::CardHealthPointRepository;

use crate::card_health_point::repository::card_health_point_repository_impl::CardHealthPointRepositoryImpl;
use crate::card_health_point::service::card_health_point_service::CardHealthPointService;
use crate::common::card_attributes::card_health_point::health_point_enum::HealthPointEnum;



pub struct CardHealthPointServiceImpl {
    card_health_point_repository: Arc<AsyncMutex<CardHealthPointRepositoryImpl>>
}

impl CardHealthPointServiceImpl {
    pub fn new(card_health_point_repository: Arc<AsyncMutex<CardHealthPointRepositoryImpl>>) -> Self {
        CardHealthPointServiceImpl {
            card_health_point_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardHealthPointServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardHealthPointServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardHealthPointServiceImpl::new(
                            CardHealthPointRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardHealthPointService for CardHealthPointServiceImpl {
    async fn get_card_health_point(&self, card_number: &i32) -> i32 {
        println!("CardHealthPointServiceImpl: get_card_health_point()");

        let card_health_point_repository_guard = self.card_health_point_repository.lock().await;
        card_health_point_repository_guard.get_card_health_point(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_health_point() {
        let card_health_point_repo_mutex = CardHealthPointRepositoryImpl::get_instance();
        let card_health_point_repo_guard = card_health_point_repo_mutex.lock().await;
        let card_number: i32 = 6;

        let card_health_point = card_health_point_repo_guard.get_card_health_point(&card_number).await;
        println!("Card Health Point: {:?}", card_health_point);

    }
}