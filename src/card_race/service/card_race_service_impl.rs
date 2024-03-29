use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_race::repository::card_race_repository::CardRaceRepository;

use crate::card_race::repository::card_race_repository_impl::CardRaceRepositoryImpl;
use crate::card_race::service::card_race_service::CardRaceService;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

pub struct CardRaceServiceImpl {
    card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>
}

impl CardRaceServiceImpl {
    pub fn new(card_race_repository: Arc<AsyncMutex<CardRaceRepositoryImpl>>) -> Self {
        CardRaceServiceImpl {
            card_race_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardRaceServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardRaceServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardRaceServiceImpl::new(
                            CardRaceRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardRaceService for CardRaceServiceImpl {
    async fn get_card_race(&self, card_number: &i32) -> RaceEnum {
        println!("CardRaceServiceImpl: get_card_race()");

        let card_race_repository_guard = self.card_race_repository.lock().await;
        card_race_repository_guard.get_card_race(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_kind() {
        let card_race_repo_mutex = CardRaceServiceImpl::get_instance();
        let card_race_repo_guard = card_race_repo_mutex.lock().await;
        let card_number: i32 = 6;

        let card_race = card_race_repo_guard.get_card_race(&card_number).await;
        println!("Card Grade: {:?}", card_race);
        assert_eq!(card_race, RaceEnum::Undead);
    }
}