use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;

use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;

use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_tomb::service::response::place_to_tomb_response::PlaceToTombResponse;

pub struct GameTombServiceImpl {
    game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>,
}

impl GameTombServiceImpl {
    pub fn new(game_tomb_repository: Arc<AsyncMutex<GameTombRepositoryImpl>>) -> Self {

        GameTombServiceImpl {
            game_tomb_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTombServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTombServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameTombServiceImpl::new(
                            GameTombRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameTombService for GameTombServiceImpl {
    async fn add_used_card_to_tomb(&mut self, place_to_tomb_request: PlaceToTombRequest) -> PlaceToTombResponse {
        println!("GameTombServiceImpl: add_used_card_to_tomb()");

        let mut game_tomb_repository_guard = self.game_tomb_repository.lock().await;
        let place_to_tomb_response = game_tomb_repository_guard.add_used_card_to_tomb(
            place_to_tomb_request.get_account_unique_id(), place_to_tomb_request.get_used_card_id());

        PlaceToTombResponse::new(place_to_tomb_response)
    }
}