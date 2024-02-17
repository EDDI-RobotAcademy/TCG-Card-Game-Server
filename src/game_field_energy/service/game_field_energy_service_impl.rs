use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::game_field_energy::repository::game_field_energy_repository::GameFieldEnergyRepository;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::request::check_field_energy_enough_to_use_request::CheckFieldEnergyEnoughToUseRequest;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::response::add_field_energy_with_amount_response::AddFieldEnergyWithAmountResponse;
use crate::game_field_energy::service::response::check_field_energy_enough_to_use_response::CheckFieldEnergyEnoughToUseResponse;
use crate::game_field_energy::service::response::remove_field_energy_with_amount_response::RemoveFieldEnergyWithAmountResponse;

pub struct GameFieldEnergyServiceImpl {
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
}

impl GameFieldEnergyServiceImpl {
    pub fn new(game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>) -> Self {
        GameFieldEnergyServiceImpl {
            game_field_energy_repository,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameFieldEnergyServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldEnergyServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldEnergyServiceImpl::new(
                            GameFieldEnergyRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameFieldEnergyService for GameFieldEnergyServiceImpl {
    async fn add_field_energy_with_amount(&self, add_field_energy_with_amount_request: AddFieldEnergyWithAmountRequest) -> AddFieldEnergyWithAmountResponse {
        println!("GameFieldEnergyServiceImpl: add_field_energy_with_amount()");

        let mut game_field_energy_repository_guard = self.game_field_energy_repository.lock().await;

        let account_unique_id = add_field_energy_with_amount_request.get_account_unique_id();
        let amount = add_field_energy_with_amount_request.get_amount_to_add();

        let addition_of_field_energy_result = game_field_energy_repository_guard.add_field_energy_with_amount(account_unique_id, amount);

        AddFieldEnergyWithAmountResponse::new(addition_of_field_energy_result)
    }

    async fn remove_field_energy_with_amount(&self, remove_field_energy_with_amount_request: RemoveFieldEnergyWithAmountRequest) -> RemoveFieldEnergyWithAmountResponse {
        println!("GameFieldEnergyServiceImpl: remove_field_energy_with_amount()");

        let mut game_field_energy_repository_guard = self.game_field_energy_repository.lock().await;

        let account_unique_id = remove_field_energy_with_amount_request.get_account_unique_id();
        let amount = remove_field_energy_with_amount_request.get_amount_to_remove();

        let removal_of_field_energy_result = game_field_energy_repository_guard.remove_field_energy_with_amount(account_unique_id, amount);

        RemoveFieldEnergyWithAmountResponse::new(removal_of_field_energy_result)
    }

    async fn check_field_energy_enough_to_use(&self, check_field_energy_enough_to_use_request: CheckFieldEnergyEnoughToUseRequest) -> CheckFieldEnergyEnoughToUseResponse {
        println!("GameFieldEnergyServiceImpl: remove_field_energy_with_amount()");

        let mut game_field_energy_repository_guard = self.game_field_energy_repository.lock().await;

        let account_unique_id = check_field_energy_enough_to_use_request.get_account_unique_id();
        let amount = check_field_energy_enough_to_use_request.get_will_be_used_amount();

        let is_field_energy_enough_result = game_field_energy_repository_guard.check_field_energy_enough_to_use(account_unique_id, amount);

        CheckFieldEnergyEnoughToUseResponse::new(is_field_energy_enough_result)
    }
}