use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_attack_point::repositoty::card_attack_point_repository::CardAttackPointRepository;

use crate::card_attack_point::repositoty::card_attack_point_repository_impl::CardAttackPointRepositoryImpl;
use crate::card_attack_point::service::card_attack_point_service::CardAttackPointService;
use crate::common::card_attributes::card_attack_point::attack_point_enum::AttackPointEnum;


pub struct CardAttackPointServiceImpl {
    card_attack_point_repository: Arc<AsyncMutex<CardAttackPointRepositoryImpl>>
}

impl CardAttackPointServiceImpl {
    pub fn new(card_attack_point_repository: Arc<AsyncMutex<CardAttackPointRepositoryImpl>>) -> Self {
        CardAttackPointServiceImpl {
            card_attack_point_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardAttackPointServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardAttackPointServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardAttackPointServiceImpl::new(
                            CardAttackPointRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardAttackPointService for CardAttackPointServiceImpl {
    async fn get_card_attack_point(&self, card_number: &i32) -> i32 {
        println!("CardAttackPointServiceImpl: get_card_attack_point()");

        let card_attack_point_repository_guard = self.card_attack_point_repository.lock().await;
        card_attack_point_repository_guard.get_card_attack_point(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_attack_point() {
        let card_attack_point_repo_mutex = CardAttackPointRepositoryImpl::get_instance();
        let card_attack_point_repo_guard = card_attack_point_repo_mutex.lock().await;
        let card_number: i32 = 6;

        let card_attack_point = card_attack_point_repo_guard.get_card_attack_point(&card_number).await;
        println!("Card Grade: {:?}", card_attack_point);

    }
}