use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::game_lost_zone::repository::game_lost_zone_repository::GameLostZoneRepository;
use crate::game_lost_zone::repository::game_lost_zone_repository_impl::GameLostZoneRepositoryImpl;
use crate::game_lost_zone::service::game_lost_zone_service::GameLostZoneService;
use crate::game_lost_zone::service::request::place_card_to_lost_zone_request::PlaceCardToLostZoneRequest;
use crate::game_lost_zone::service::response::place_card_to_lost_zone_response::PlaceCardToLostZoneResponse;

pub struct GameLostZoneServiceImpl {
    game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>,
}

impl GameLostZoneServiceImpl {
    pub fn new(game_lost_zone_repository: Arc<AsyncMutex<GameLostZoneRepositoryImpl>>) -> Self {
        GameLostZoneServiceImpl {
            game_lost_zone_repository,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameLostZoneServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameLostZoneServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameLostZoneServiceImpl::new(
                            GameLostZoneRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameLostZoneService for GameLostZoneServiceImpl {
    async fn place_card_to_lost_zone(&self, place_card_to_lost_zone_request: PlaceCardToLostZoneRequest) -> PlaceCardToLostZoneResponse {
        let mut game_lost_zone_repository_guard = self.game_lost_zone_repository.lock().await;
        let result = game_lost_zone_repository_guard.add_lost_zone_card(
            place_card_to_lost_zone_request.get_account_unique_id(),
            place_card_to_lost_zone_request.get_lost_card_id());

        PlaceCardToLostZoneResponse::new(result)
    }
}