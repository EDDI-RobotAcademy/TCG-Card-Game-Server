use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::{Death, Survival};
use crate::game_main_character::repository::game_main_character_repository::GameMainCharacterRepository;
use crate::game_main_character::repository::game_main_character_repository_impl::GameMainCharacterRepositoryImpl;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_main_character::service::request::check_main_character_of_account_unique_id_request::CheckMainCharacterOfAccountUniqueIdRequest;
use crate::game_main_character::service::request::get_current_main_character_health_point_request::GetCurrentMainCharacterHealthPointRequest;
use crate::game_main_character::service::response::apply_damage_to_main_character_response::ApplyDamageToMainCharacterResponse;
use crate::game_main_character::service::response::check_main_character_of_account_unique_id_response::CheckMainCharacterOfAccountUniqueIdResponse;
use crate::game_main_character::service::response::get_current_main_character_health_point_response::GetCurrentMainCharacterHealthPointResponse;

pub struct GameMainCharacterServiceImpl {
    game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
}

impl GameMainCharacterServiceImpl {
    pub fn new(game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>) -> Self {
        GameMainCharacterServiceImpl {
            game_main_character_repository,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameMainCharacterServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameMainCharacterServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameMainCharacterServiceImpl::new(
                            GameMainCharacterRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameMainCharacterService for GameMainCharacterServiceImpl {
    async fn apply_damage_to_main_character(&self, apply_damage_to_main_character_request: ApplyDamageToMainCharacterRequest) -> ApplyDamageToMainCharacterResponse {
        println!("GameMainCharacterServiceImpl: apply_damage_to_main_character()");

        let mut game_main_character_repository_guard = self.game_main_character_repository.lock().await;
        let apply_damage_result = game_main_character_repository_guard.apply_damage_to_main_character(
            apply_damage_to_main_character_request.get_account_unique_id(),
            apply_damage_to_main_character_request.get_damage());

        ApplyDamageToMainCharacterResponse::new(apply_damage_result)
    }

    async fn check_main_character_of_account_unique_id(&mut self, check_main_character_of_account_unique_id_request: CheckMainCharacterOfAccountUniqueIdRequest) -> CheckMainCharacterOfAccountUniqueIdResponse {
        println!("GameFieldUnitServiceImpl: check_main_character_of_account_unique_id()");

        let account_unique_id = check_main_character_of_account_unique_id_request.get_account_unique_id();

        let mut game_main_character_repository_guard = self.game_main_character_repository.lock().await;
        let mut health_point_of_main_character =  game_main_character_repository_guard.get_health_point_of_main_character_by_account_unique_id(account_unique_id);

        if health_point_of_main_character <= 0 {
            let mut game_main_character_repository_guard = self.game_main_character_repository.lock().await;
            let mut main_character_map_by_account_unique_id = game_main_character_repository_guard.get_game_main_character_map();
            let mut main_character_account_unique_id = main_character_map_by_account_unique_id.get_mut(&account_unique_id).unwrap();
            main_character_account_unique_id.set_status(Death);
            return CheckMainCharacterOfAccountUniqueIdResponse::new(Death);
        }
        return CheckMainCharacterOfAccountUniqueIdResponse::new(Survival);
    }

    async fn get_current_main_character_health_point(
        &mut self, get_current_main_character_health_point_request: GetCurrentMainCharacterHealthPointRequest)
        -> GetCurrentMainCharacterHealthPointResponse {

        println!("GameFieldUnitServiceImpl: check_main_character_of_account_unique_id()");

        let mut game_main_character_repository_guard =
            self.game_main_character_repository.lock().await;

        let health_point =
            game_main_character_repository_guard.get_health_point_of_main_character_by_account_unique_id(
                get_current_main_character_health_point_request.get_account_unique_id());

        GetCurrentMainCharacterHealthPointResponse::new(health_point)
    }
}