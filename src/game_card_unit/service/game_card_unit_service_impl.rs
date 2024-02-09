use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;

use crate::game_card_support::service::game_card_support_service::GameCardSupportService;
use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::repository::game_card_unit_repository::GameCardUnitRepository;

use crate::game_card_unit::repository::game_card_unit_repository_impl::GameCardUnitRepositoryImpl;
use crate::game_card_unit::service::game_card_unit_service::GameCardUnitService;
use crate::game_card_unit::service::request::summary_unit_card_info_request::SummaryUnitCardInfoRequest;
use crate::game_card_unit::service::response::summary_unit_card_info_response::SummaryUnitCardInfoResponse;

pub struct GameCardUnitServiceImpl {
    game_card_unit_repository: Arc<AsyncMutex<GameCardUnitRepositoryImpl>>,
}

impl GameCardUnitServiceImpl {
    pub fn new(game_card_unit_repository: Arc<AsyncMutex<GameCardUnitRepositoryImpl>>,
    ) -> Self {
        GameCardUnitServiceImpl {
            game_card_unit_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardUnitServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardUnitServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardUnitServiceImpl::new(
                            GameCardUnitRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardUnitService for GameCardUnitServiceImpl {
    async fn summary_unit_card(&mut self, summary_unit_card_info_request: SummaryUnitCardInfoRequest) -> SummaryUnitCardInfoResponse {
        println!("GameCardUnitServiceImpl: summary_unit_card()");

        let game_card_unit_repository_guard = self.game_card_unit_repository.lock().await;
        let game_card_unit_info = unsafe {
            game_card_unit_repository_guard.call_unit_card_repository_handler(summary_unit_card_info_request.get_unit_card_id())
        };

        return SummaryUnitCardInfoResponse::from_game_card_unit_info(game_card_unit_info)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::{Human, Undead};
    use crate::game_card_support::repository::game_card_support_repository_impl::GameCardSupportRepositoryImpl;
    use super::*;
    use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

    #[tokio::test]
    async fn test_game_card_unit_service() {
        let service = GameCardUnitServiceImpl::get_instance();
        let mut service_guard = service.lock().await;

        let unit_id = 6;
        let calculate_unit_effect_request = SummaryUnitCardInfoRequest::new(unit_id);
        let unit_effect_result = service_guard.summary_unit_card(calculate_unit_effect_request).await;

        assert_eq!(unit_effect_result.get_unit_race(), RaceEnum::Human);

        println!("unit_effect_result: {:?}", unit_effect_result);
    }
}
