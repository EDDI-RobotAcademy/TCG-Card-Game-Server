use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_winner_check::repository::game_winner_check_repository_impl::GameWinnerCheckRepositoryImpl;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::game_main_character::repository::game_main_character_repository::GameMainCharacterRepository;
use crate::game_main_character::repository::game_main_character_repository_impl::GameMainCharacterRepositoryImpl;
use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum::{Loser, Winner};
use crate::game_winner_check::repository::game_winner_check_repository::GameWinnerCheckRepository;
use crate::game_winner_check::service::game_winner_check_service::GameWinnerCheckService;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameWinnerCheckServiceImpl {
    game_winner_check_repository: Arc<AsyncMutex<GameWinnerCheckRepositoryImpl>>,
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl GameWinnerCheckServiceImpl {
    pub fn new(game_winner_check_repository: Arc<AsyncMutex<GameWinnerCheckRepositoryImpl>>,
               battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               game_main_character_repository: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    ) -> Self {
        GameWinnerCheckServiceImpl {
            game_winner_check_repository,
            battle_room_repository,
            game_main_character_repository,
            redis_in_memory_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameWinnerCheckServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameWinnerCheckServiceImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameWinnerCheckServiceImpl::new(
                            GameWinnerCheckRepositoryImpl::get_instance(),
                            BattleRoomRepositoryImpl::get_instance(),
                            GameMainCharacterRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                       )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameWinnerCheckService for GameWinnerCheckServiceImpl {

    async fn check_health_of_main_character_for_setting_game_winner(&mut self, check_main_character_request: CheckMainCharacterRequest) {
        println!("GameWinnerCheckServiceImpl: check_health_of_main_character_for_setting_game_winner()");

        let account_unique_id = check_main_character_request.get_account_unique_id();

        let mut game_main_character_guard = self.game_main_character_repository.lock().await;
        let mut main_character_health = game_main_character_guard.get_health_point_of_main_character_by_account_unique_id(account_unique_id);
        drop(game_main_character_guard);

        if main_character_health <= 0 {

            let battle_room_repository_guard = self.battle_room_repository.lock().await;
            let opponent_unique_id = battle_room_repository_guard.find_opponent_unique_id(account_unique_id).await.unwrap();
            drop(battle_room_repository_guard);

            let mut game_winner_check_guard = self.game_winner_check_repository.lock().await;
            game_winner_check_guard.create_finish_position_object(account_unique_id, Loser);
            game_winner_check_guard.add_finish_position_object(opponent_unique_id, Winner);
            drop(game_winner_check_guard);
        }
    }
}