use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;

use crate::game_field_unit::repository::game_field_unit_repository_impl::GameFieldUnitRepositoryImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::request::attach_energy_to_unit_request::AttachEnergyToUnitRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_field_unit_request::AttachMultipleEnergyToFieldUnitRequest;
use crate::game_field_unit::service::response::attach_energy_to_unit_response::AttachEnergyToUnitResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_field_unit_response::AttachMultipleEnergyToFieldUnitResponse;


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
    async fn attach_multiple_energy_to_game_field_unit(&mut self, attach_multiple_energy_to_field_unit_request: AttachMultipleEnergyToFieldUnitRequest) -> AttachMultipleEnergyToFieldUnitResponse {
        println!("GameFieldUnitServiceImpl: attach_multiple_energy_to_game_field_unit()");

        let mut game_field_unit_repository_guard = self.game_field_unit_repository.lock().await;
        let response = game_field_unit_repository_guard.attach_multiple_energy_to_game_field_unit(
            attach_multiple_energy_to_field_unit_request.get_account_unique_id(),
            attach_multiple_energy_to_field_unit_request.get_unit_number(),
            attach_multiple_energy_to_field_unit_request.get_boost_race(),
            attach_multiple_energy_to_field_unit_request.get_energy_count());

        AttachMultipleEnergyToFieldUnitResponse::new(response)
    }

    // fn attach_multiple_energy_to_game_field_unit(&mut self, attach_energy_to_unit_request: AttachEnergyToUnitRequest) -> AttachEnergyToUnitResponse {
    //     println!("GameFieldUnitServiceImpl: attach_multiple_energy_to_game_field_unit()");
    //
    //     AttachEnergyToUnitResponse::new(true)
    // }
}