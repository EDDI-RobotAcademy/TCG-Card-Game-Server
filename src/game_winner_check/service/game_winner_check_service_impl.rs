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
use crate::game_winner_check::service::request::surrender_request::SurrenderRequest;
use crate::game_winner_check::service::response::surrender_response::SurrenderResponse;
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

     async fn set_game_winner_by_surrender(&mut self, surrender_request: SurrenderRequest) -> SurrenderResponse {

        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_unique_id_sting = redis_repository_guard.get(surrender_request.get_session_id()).await.unwrap();
        let account_unique_id: i32 = account_unique_id_sting.parse().expect("Failed to parse account_unique_id_string as i32");
        drop(redis_repository_guard);

        let battle_room_repository_guard = self.battle_room_repository.lock().await;
         println!("account_unique_id: {:?}", account_unique_id);
        let opponent_unique_id = battle_room_repository_guard.find_opponent_unique_id(account_unique_id).await.unwrap();
        drop(battle_room_repository_guard);

        let mut game_winner_check_guard = self.game_winner_check_repository.lock().await;
        game_winner_check_guard.create_finish_position_object(account_unique_id, Loser);
        game_winner_check_guard.add_finish_position_object(opponent_unique_id, Winner);
        drop(game_winner_check_guard);

         return SurrenderResponse::new(true)
    }
}

#[cfg(test)]
mod cfg_test {
    use super::*;
    use tokio::test;
    use uuid::Uuid;
    use crate::account::repository::account_repository::AccountRepository;
    use crate::account::repository::account_repository_impl::AccountRepositoryImpl;

    use crate::account::service::account_service::AccountService;
    use crate::account::service::account_service_impl::AccountServiceImpl;
    use crate::account::service::request::account_delete_request::AccountDeleteRequest;
    use crate::account::service::request::account_register_request::AccountRegisterRequest;
    use crate::game_winner_check::repository::game_winner_check_repository_impl::GameWinnerCheckRepositoryImpl;

    #[test]
    async fn async_test_surrender_request() {
        let repository = GameWinnerCheckRepositoryImpl::new();

        // 본인 계정 생성
        println!("본인 계정 생성");
        let mut account_service_mutex = AccountServiceImpl::get_instance();
        let mut account_service_guard = account_service_mutex.lock().await;

        let test_account_id = "surrender_test9";
        let test_account_pw = "surrender_test9";
        let string_test_account_pw = test_account_pw.to_string();

        let test_account = AccountRegisterRequest::new(test_account_id, string_test_account_pw);
        account_service_guard.account_register(test_account).await;
        drop(account_service_guard);
        drop(account_service_mutex);

        // 상대 계정 생성
        println!("상대 계정 생성");
        let mut account_service_mutex = AccountServiceImpl::get_instance();
        let mut account_service_guard = account_service_mutex.lock().await;

        let test_opponent_id = "surrender_test10";
        let test_opponent_pw = "surrender_test10";
        let string_test_opponent_pw = test_opponent_pw.to_string();

        let test_account = AccountRegisterRequest::new(test_opponent_id, string_test_opponent_pw);
        account_service_guard.account_register(test_account).await;
        drop(account_service_guard);
        drop(account_service_mutex);

        // 본인 로그인 redis_token 생성
        println!("본인 redis_token 생성");
        let mut account_repository_mutex = AccountRepositoryImpl::get_instance();
        let mut account_repository_guard = account_repository_mutex.lock().await;

        let found_account = account_repository_guard.find_by_user_id(test_account_id).await.unwrap().unwrap();
        drop(account_repository_guard);
        drop(account_repository_mutex);

        let mut redis_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_repository_guard = redis_repository_mutex.lock().await;

        let redis_token_account_id = Uuid::new_v4();
        redis_repository_guard.set_with_expired_time(&*redis_token_account_id.to_string(), &found_account.id.to_string(), Some(60)).await;
        drop(redis_repository_guard);
        drop(redis_repository_mutex);

        // 상대 로그인 redis_token 생성
        println!("상대 redis_token 생성");
        let mut account_repository_mutex = AccountRepositoryImpl::get_instance();
        let mut account_repository_guard = account_repository_mutex.lock().await;

        let found_opponent = account_repository_guard.find_by_user_id(test_opponent_id).await.unwrap().unwrap();
        drop(account_repository_guard);
        drop(account_repository_mutex);

        let mut redis_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_repository_guard = redis_repository_mutex.lock().await;

        let redis_token_opponent_id = Uuid::new_v4();
        redis_repository_guard.set_with_expired_time(&*redis_token_opponent_id.to_string(), &found_opponent.id.to_string(), Some(120)).await;

        // 본인 account_unique_id 추출
        println!("본인 account_unique_id 추출");
        let account_unique_id_string = redis_repository_guard.get(&*redis_token_account_id.to_string()).await.unwrap();
        println!("account_unique_id_string: {:?}", account_unique_id_string);
        let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

        // 상대 opponent_unique_id 추출
        println!("상대 opponent_unique_id_sting 추출");
        let opponent_unique_id_string = redis_repository_guard.get(&*redis_token_opponent_id.to_string()).await.unwrap();
        let opponent_unique_id: i32 = opponent_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
        drop(redis_repository_guard);
        drop(redis_repository_mutex);

        // 배틀룸 생성
        println!("배틀룸 생성");
        let mut battle_room_repository_mutex = BattleRoomRepositoryImpl::get_instance();
        let battle_room_repository_guard = battle_room_repository_mutex.lock().await;

        let mut account_vector: Vec<i32> = Vec::new();
        account_vector.push(account_unique_id);
        account_vector.push(opponent_unique_id);
        battle_room_repository_guard.set_players_to_battle_room(account_vector).await.expect("e");
        let battle_room_status = battle_room_repository_guard.get_battle_room_account_hash();

        drop(battle_room_repository_guard);
        drop(battle_room_repository_mutex);

        let mut game_winner_service_mutex = GameWinnerCheckServiceImpl::get_instance();
        let mut game_winner_service_guard = game_winner_service_mutex.lock().await;

        println!("account_id 가 항복 요청으로 승패 셋팅");
        println!("항복요청하는 session_id: {:?}", redis_token_account_id.to_string());
        let surrender_test_quest = SurrenderRequest::new(redis_token_account_id.to_string());
        game_winner_service_guard.set_game_winner_by_surrender(surrender_test_quest).await;
        drop(game_winner_service_guard);
        drop(game_winner_service_mutex);

        println!("승패 셋팅 값에서 계정별 finish position 확인");
        let mut game_winner_repository_mutex = GameWinnerCheckRepositoryImpl::get_instance();
        let mut game_winner_repository_guard = game_winner_repository_mutex.lock().await;
        let finish_account_id_position = game_winner_repository_guard.get_game_finish_position_map().get(&account_unique_id).unwrap();

        println!("redis_token_account_id: {:?}", redis_token_account_id.to_string());
        println!("account_unique_id: {:?}", account_unique_id);
        println!("account_unique_id_finish_position: {:?}", finish_account_id_position);
        drop(game_winner_repository_guard);
        drop(game_winner_repository_mutex);

        let mut game_winner_repository_mutex = GameWinnerCheckRepositoryImpl::get_instance();
        let mut game_winner_repository_guard = game_winner_repository_mutex.lock().await;
        let finish_opponent_id_position = game_winner_repository_guard.get_game_finish_position_map().get(&opponent_unique_id).unwrap();

        println!("redis_token_opponent_id: {:?}", redis_token_account_id.to_string());
        println!("opponent_unique_id: {:?}", opponent_unique_id);
        println!("opponent_unique_id_finish_position: {:?}", finish_opponent_id_position);
        drop(game_winner_repository_guard);
        drop(game_winner_repository_mutex);
    }
}