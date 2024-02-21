use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_tomb::repository::game_tomb_repository::GameTombRepository;

use crate::game_tomb::repository::game_tomb_repository_impl::GameTombRepositoryImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::request::add_dead_unit_list_to_tomb_request::AddDeadUnitListToTombRequest;

use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_tomb::service::response::add_dead_unit_list_to_tomb_response::AddDeadUnitListToTombResponse;
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
    async fn add_dead_unit_to_tomb(&mut self, place_to_tomb_request: PlaceToTombRequest) -> PlaceToTombResponse {
        println!("GameTombServiceImpl: add_dead_unit_to_tomb()");

        let mut game_tomb_repository_guard = self.game_tomb_repository.lock().await;
        let place_to_tomb_response = game_tomb_repository_guard.add_dead_unit_to_tomb(
            place_to_tomb_request.get_account_unique_id(), place_to_tomb_request.get_used_card_id());

        PlaceToTombResponse::new(place_to_tomb_response)
    }

    async fn add_dead_unit_list_to_tomb(&mut self, add_dead_unit_list_to_tomb_request: AddDeadUnitListToTombRequest) -> AddDeadUnitListToTombResponse {
        println!("GameTombServiceImpl: add_dead_unit_list_to_tomb()");

        let mut game_tomb_repository_guard = self.game_tomb_repository.lock().await;
        let dead_unit_list = add_dead_unit_list_to_tomb_request.get_dead_unit_list().clone();
        for dead_unit in dead_unit_list {
            let result =
                game_tomb_repository_guard.add_dead_unit_to_tomb(
                    add_dead_unit_list_to_tomb_request.get_account_unique_id(), dead_unit);
            if result == false {
                return AddDeadUnitListToTombResponse::new(result)
            }
        }

        AddDeadUnitListToTombResponse::new(true)
    }
}