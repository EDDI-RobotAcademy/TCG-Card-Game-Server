use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use std::time::Duration;
use diesel::row::NamedRow;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use crate::account_card::entity::account_card::account_cards::account_id;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;

use crate::game_deck::repository::game_deck_repository::GameDeckRepository;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_field_energy::repository::game_field_energy_repository::GameFieldEnergyRepository;
use crate::game_field_energy::repository::game_field_energy_repository_impl::GameFieldEnergyRepositoryImpl;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;

use crate::rock_paper_scissors::service::rock_paper_scissors_service::RockPaperScissorsService;
use crate::rock_paper_scissors::service::request::register_rock_paper_scissors_wait_hash_request::RegisterRockPaperScissorsWaitHashRequest;
use crate::rock_paper_scissors::service::response::register_rock_paper_scissors_wait_hash_response::RegisterRockPaperScissorsWaitHashResponse;
use crate::rock_paper_scissors::repository::rock_paper_scissors_repository::RockPaperScissorsRepository;
use crate::rock_paper_scissors::repository::rock_paper_scissors_repository_impl::{RockPaperScissorsRepositoryImpl};
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult::WIN;
use crate::rock_paper_scissors::service::request::check_opponent_choice_request::CheckOpponentChoiceRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::{CheckRockPaperScissorsWinnerRequest};

use crate::rock_paper_scissors::service::response::check_opponent_choice_response::CheckOpponentHashmapResponse;
use crate::rock_paper_scissors::service::response::check_rock_paper_scissors_winner_response::CheckRockPaperScissorsWinnerResponse;
use crate::rock_paper_scissors_waiting_timer::repository::rock_paper_scissors_waiting_timer_repository::RockPaperScissorsWaitingTimerRepository;
use crate::rock_paper_scissors_waiting_timer::repository::rock_paper_scissors_waiting_timer_repository_impl::RockPaperScissorsWaitingTimerRepositoryImpl;

pub struct RockPaperScissorsServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    rock_paper_scissors_repository: Arc<AsyncMutex<RockPaperScissorsRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
    game_turn_repository:Arc<AsyncMutex<GameTurnRepositoryImpl>>,
    game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    rock_paper_scissors_waiting_timer_repository: Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>>
}

impl RockPaperScissorsServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               rock_paper_scissors_repository: Arc<AsyncMutex<RockPaperScissorsRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
               game_turn_repository:Arc<AsyncMutex<GameTurnRepositoryImpl>>,
               game_field_energy_repository: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>>,
               game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               rock_paper_scissors_waiting_timer_repository: Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>>

    ) -> Self {

        RockPaperScissorsServiceImpl {
            redis_in_memory_repository,
            rock_paper_scissors_repository,
            match_waiting_timer_repository,
            game_turn_repository,
            game_field_energy_repository,
            game_deck_repository,
            rock_paper_scissors_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockPaperScissorsServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockPaperScissorsServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RockPaperScissorsServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            RockPaperScissorsRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance(),
                            GameTurnRepositoryImpl::get_instance(),
                            GameFieldEnergyRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
                            RockPaperScissorsWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }
    async fn is_rock_paper_scissors_waiting_timer_expired(&self, account_unique_id: i32) -> bool {
        let mut rock_paper_scissors_waiting_timer_repository_mutex =
            self.rock_paper_scissors_waiting_timer_repository.lock().await;
        rock_paper_scissors_waiting_timer_repository_mutex.
            check_rock_paper_scissors_waiting_timer_expired(account_unique_id, Duration::from_secs(60)).await
    }
}

#[async_trait]
impl RockPaperScissorsService for RockPaperScissorsServiceImpl {
    async fn register_rock_paper_scissors_wait_hash(
        &self, register_rock_paper_scissors_wait_hash_request: RegisterRockPaperScissorsWaitHashRequest)
        -> RegisterRockPaperScissorsWaitHashResponse {

        println!("RockPaperScissorsServiceImpl: register_rock_paper_scissors_wait_hash()");

        let account_unique_id = register_rock_paper_scissors_wait_hash_request.get_account_unique_id();
        let opponent_unique_id = register_rock_paper_scissors_wait_hash_request.get_opponent_unique_id();
        let choice = register_rock_paper_scissors_wait_hash_request.get_choice().to_string();

        let rock_paper_scissors_repository_guard =
            self.rock_paper_scissors_repository.lock().await;

        let response =
            rock_paper_scissors_repository_guard
                .register_choice_repo(account_unique_id, choice).await;

        rock_paper_scissors_repository_guard
            .change_draw_choices_repo(
                account_unique_id, opponent_unique_id).await;

        RegisterRockPaperScissorsWaitHashResponse::new(response)
        }


    async fn check_rock_paper_scissors_winner(
        &self, check_rock_paper_scissors_winner_request: CheckRockPaperScissorsWinnerRequest)
        -> CheckRockPaperScissorsWinnerResponse {

        println!("RockPaperScissorsServiceImpl: check_rock_paper_scissors_winner()");

        let account_unique_id = check_rock_paper_scissors_winner_request.get_account_unique_id();
        let opponent_unique_id = check_rock_paper_scissors_winner_request.get_opponent_unique_id();

        let rock_paper_scissors_repository_guard =
            self.rock_paper_scissors_repository.lock().await;

        let rock_paper_scissors_result =
            rock_paper_scissors_repository_guard
                .check_result_repo(account_unique_id, opponent_unique_id).await;

        drop(rock_paper_scissors_repository_guard);

        // let mut game_turn_repository_guard =
        //     self.game_turn_repository.lock().await;
        // let mut game_field_energy_repository_guard =
        //     self.game_field_energy_repository.lock().await;
        // let mut game_deck_repository_guard=
        //     self.game_deck_repository.lock().await;

        // TODO: 여기서 진행하는 것이 아닌 Service 로 진행해야 함 (to notify)
        // if rock_paper_scissors_result == WIN {
        //     game_turn_repository_guard.next_game_turn(account_unique_id);
        //     drop(game_turn_repository_guard);
        //     game_field_energy_repository_guard.add_field_energy_with_amount(account_unique_id,1);
        //     drop(game_field_energy_repository_guard);
        //     game_deck_repository_guard.draw_deck_card(account_unique_id,1);
        //     drop(game_deck_repository_guard);
        //     return CheckRockPaperScissorsWinnerResponse::new(rock_paper_scissors_result)
        // }

        CheckRockPaperScissorsWinnerResponse::new(rock_paper_scissors_result)
    }
}





