use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_main_character::repository::game_main_character_repository::GameMainCharacterRepository;
use crate::game_main_character::repository::game_main_character_repository_impl::GameMainCharacterRepositoryImpl;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_main_character::service::response::apply_damage_to_main_character_response::ApplyDamageToMainCharacterResponse;

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
}