use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use tokio::time::sleep;
use std::time::{Duration, Instant};
use diesel::row::NamedRow;


use rand::{Rng, SeedableRng};
use rand::rngs::{OsRng, StdRng};
use rand::seq::index::sample;


use crate::account_card::entity::account_card::account_cards::account_id;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;
use crate::game_turn::repository::game_turn_repository_impl::GameTurnRepositoryImpl;
use crate::match_waiting_timer::entity::match_waiting_timer::MatchWaitingTimer;

use crate::rockpaperscissors::service::rockpaperscissors_service::RockpaperscissorsService;
use crate::rockpaperscissors::service::request::wait_hashmap_request::WaitHashmapRequest;
use crate::rockpaperscissors::service::response::wait_hashmap_response::WaitHashmapResponse;
use crate::rockpaperscissors::repository::rockpaperscissors_repository::RockpaperscissorsRepository;
use crate::rockpaperscissors::repository::rockpaperscissors_repository_impl::RockpaperscissorsRepositoryImpl;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::rockpaperscissors::service::request::check_draw_choice_request::CheckDrawChoiceRequest;
use crate::rockpaperscissors::service::request::check_rockpaperscissors_winner_request::{CheckRockpaperscissorsWinnerRequest};
use crate::rockpaperscissors::service::response::check_draw_choice_response;
use crate::rockpaperscissors::service::response::check_rockpaperscissors_winner_response::CheckRockpaperscissorsWinnerResponse;

pub struct RockpaperscissorsServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    rockpaperscissors_repository: Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
    game_turn_repository:Arc<AsyncMutex<GameTurnRepositoryImpl>>,
}

impl RockpaperscissorsServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               rockpaperscissors_repository: Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
               game_turn_repository:Arc<AsyncMutex<GameTurnRepositoryImpl>>,

    ) -> Self {

        RockpaperscissorsServiceImpl {
            redis_in_memory_repository,
            rockpaperscissors_repository,
            match_waiting_timer_repository,
            game_turn_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockpaperscissorsServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockpaperscissorsServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RockpaperscissorsServiceImpl::new(
                            RedisInMemoryRepositoryImpl::get_instance(),
                            RockpaperscissorsRepositoryImpl::get_instance(),
                            MatchWaitingTimerRepositoryImpl::get_instance(),
                            GameTurnRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }
    async fn is_match_waiting_timer_expired(&self, account_unique_id: i32) -> bool {
        let mut match_waiting_timer_repository_mutex = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository_mutex.check_match_waiting_timer_expired(account_unique_id, Duration::from_secs(60)).await
    }



}

#[async_trait]
impl RockpaperscissorsService for RockpaperscissorsServiceImpl {


    async fn insert_player_data_to_hashmap(&self, insert_player_data_to_hashmap_request: WaitHashmapRequest) -> WaitHashmapResponse {
        println!("RockpaperscissorsServiceImpl: insert_player_data_to_hashmap()");
        let account_unique_id = insert_player_data_to_hashmap_request.get_account_unique_id();
        let choice=insert_player_data_to_hashmap_request.get_choice().to_string();
        let mut player_map: HashMap<String, String> = Default::default();
        player_map.insert(account_unique_id.to_string(), choice);

        let rockpaperscissors_repository = self.rockpaperscissors_repository.lock().await;

        let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;

        let mut response = rockpaperscissors_repository.insert_player_hashmap_for_wait(player_map).await;

        if response.is_ok() {
            return WaitHashmapResponse::new(true)
        }

        return WaitHashmapResponse::new(false)

    }

    async fn check_draw_choice(&self, check_draw_choice_request: CheckDrawChoiceRequest){
        let account_unique_id=check_draw_choice_request.get_account_unique_id().to_string();
        let opponent_id=check_draw_choice_request.get_opponent_id().to_string();
        let choices = vec!["Rock", "Paper", "Scissors"];
        let mut rng = StdRng::seed_from_u64(42); // 임의의 시드값 사용

        // "Rock", "Paper", "Scissors" 중에서 중복되지 않게 2개 선택
        let sampled_indices = sample(&mut rng, choices.len(), 2);
        let selected_choices: Vec<&str> = sampled_indices.into_iter().map(|index| choices[index]).collect();
        let rockpaperscissors_repository_guard = self.rockpaperscissors_repository.lock().await;
        // let wait_hashmap_clone_mutex = rockpaperscissors_repository_guard.get_wait_hashmap();
        // let wait_hashmap_clone_guard = wait_hashmap_clone_mutex.lock().await;
        rockpaperscissors_repository_guard.change_draw_choice_repo(account_unique_id,opponent_id,selected_choices);
        // let mut my_choice=wait_hashmap_clone_guard.get_player_hashmap(account_unique_id.to_string()).await.unwrap();
        // let mut opponent_choice=wait_hashmap_clone_guard.get_player_hashmap(opponent_id.to_string()).await.unwrap();

    }

    async fn check_rockpaperscissors_winner(&self, check_rockpaperscissors_winner_request: CheckRockpaperscissorsWinnerRequest) -> CheckRockpaperscissorsWinnerResponse {
        println!("RockpaperscissorsServiceImpl: check_rockpaperscissors_winner()");
        let account_unique_id=check_rockpaperscissors_winner_request.get_account_unique_id();
        let opponent_id=check_rockpaperscissors_winner_request.get_opponent_id();
        let rockpaperscissors_repository_guard = self.rockpaperscissors_repository.lock().await;
        let wait_hashmap_clone_mutex = rockpaperscissors_repository_guard.get_wait_hashmap();
        let wait_hashmap_clone_guard = wait_hashmap_clone_mutex.lock().await;


        let mut my_choice=wait_hashmap_clone_guard.get_player_hashmap(account_unique_id.to_string()).await.unwrap();
        let mut opponent_choice=wait_hashmap_clone_guard.get_player_hashmap(opponent_id.to_string()).await.unwrap();

        let mut rng = OsRng::default();
        let random_bool: bool = rng.gen();
        let am_i_win = match (my_choice.as_str(), opponent_choice.as_str()) {
            ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => true,
            ("Scissors", "Rock") | ("Rock", "Paper") | ("Paper", "Scissors") => false,
            _ => random_bool,
        };
        drop(rockpaperscissors_repository_guard);
        let mut game_turn_repository_guard = self.game_turn_repository.lock().await;

        if am_i_win==true
        {
            game_turn_repository_guard.next_game_turn(account_unique_id);
        }
        drop(game_turn_repository_guard);

        CheckRockpaperscissorsWinnerResponse::new(am_i_win)
    }
}





