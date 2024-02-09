use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;
use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;

use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;

use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;

use crate::game_field_unit::service::response::add_unit_to_game_field_response::AddUnitToGameFieldResponse;
use crate::game_field_unit::service::response::attach_single_energy_to_unit_index_response::AttachSingleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_unit_index_response::AttachMultipleEnergyToUnitIndexResponse;
use crate::game_field_unit::service::response::find_target_unit_id_by_index_response::FindTargetUnitIdByIndexResponse;


pub struct GameFieldUnitServiceImpl {
    game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>,
}

impl GameFieldUnitServiceImpl {
    pub fn new(game_field_unit_repository: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>>) -> Self {

        GameFieldUnitServiceImpl {
            game_field_unit_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitServiceImpl::new(
                            GameFieldUnitRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameFieldUnitService for GameFieldUnitServiceImpl {

    async fn add_unit_to_game_field(&mut self, add_unit_to_game_field_request: AddUnitToGameFieldRequest) -> AddUnitToGameFieldResponse {
        println!("GameFieldUnitServiceImpl: attach_multiple_energy_to_game_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.add_unit_to_game_field(
            add_unit_to_game_field_request.get_account_unique_id(),
            add_unit_to_game_field_request.get_unit_card_id(),
            add_unit_to_game_field_request.get_unit_race(),
            add_unit_to_game_field_request.get_unit_grade(),
            add_unit_to_game_field_request.get_unit_attack_point(),
            add_unit_to_game_field_request.get_unit_health_point(),
            add_unit_to_game_field_request.get_unit_attack_required_energy(),
            add_unit_to_game_field_request.has_third_passive_skill(),
            add_unit_to_game_field_request.has_second_passive_skill(),
            add_unit_to_game_field_request.has_third_passive_skill());

        AddUnitToGameFieldResponse::new(response)
    }

    async fn attach_energy_to_field_unit_index(&mut self, attach_energy_to_unit_index_request: AttachSingleEnergyToUnitIndexRequest) -> AttachSingleEnergyToUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: attach_energy_to_field_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_multiple_energy_to_indexed_unit(
            attach_energy_to_unit_index_request.get_account_unique_id(),
            attach_energy_to_unit_index_request.get_unit_card_index(),
            attach_energy_to_unit_index_request.get_race_enum(),
            attach_energy_to_unit_index_request.get_quantity());

        AttachSingleEnergyToUnitIndexResponse::new(true)
    }

    async fn attach_multiple_energy_to_field_unit_index(&mut self, attach_multiple_energy_to_unit_index_request: AttachMultipleEnergyToUnitIndexRequest) -> AttachMultipleEnergyToUnitIndexResponse {
        println!("GameFieldUnitServiceImpl: attach_energy_to_field_unit_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_multiple_energy_to_indexed_unit(
            attach_multiple_energy_to_unit_index_request.get_account_unique_id(),
            attach_multiple_energy_to_unit_index_request.get_unit_card_index(),
            attach_multiple_energy_to_unit_index_request.get_race_enum(),
            attach_multiple_energy_to_unit_index_request.get_quantity());

        AttachMultipleEnergyToUnitIndexResponse::new(true)
    }

    async fn find_target_unit_id_by_index(&mut self, find_target_unit_id_by_index_request: FindTargetUnitIdByIndexRequest) -> FindTargetUnitIdByIndexResponse {
        println!("GameFieldUnitServiceImpl: find_target_unit_id_by_index()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let found_target_unit_id = game_field_unit_repository_guard.find_target_unit_id_by_index(
            find_target_unit_id_by_index_request.get_opponent_unique_id(),
            find_target_unit_id_by_index_request.get_opponent_target_unit_index());

        FindTargetUnitIdByIndexResponse::new(found_target_unit_id)
    }
}
