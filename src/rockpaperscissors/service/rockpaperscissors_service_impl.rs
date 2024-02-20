use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use tokio::time::sleep;
use std::time::{Duration, Instant};
use diesel::row::NamedRow;


use rand::Rng;


use crate::account_card::entity::account_card::account_cards::account_id;
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
use crate::rockpaperscissors::service::request::check_winner_request::CheckWinnerRequest;
use crate::rockpaperscissors::service::response::check_winner_response::CheckWinnerResponse;

pub struct RockpaperscissorsServiceImpl {
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    rockpaperscissors_repository: Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>>,
    match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,
}

impl RockpaperscissorsServiceImpl {
    pub fn new(redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               rockpaperscissors_repository: Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>>,
               match_waiting_timer_repository: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>>,

    ) -> Self {

        RockpaperscissorsServiceImpl {
            redis_in_memory_repository,
            rockpaperscissors_repository,
            match_waiting_timer_repository
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
                            MatchWaitingTimerRepositoryImpl::get_instance())));
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


    async fn insert_player_data_to_hashmap(&self, wait_queue_request: WaitHashmapRequest) -> WaitHashmapResponse {
        println!("RockpaperscissorsServiceImpl: push_player_data_to_hashmap()");
        let account_unique_id = wait_queue_request.get_account_unique_id();
        let choice=wait_queue_request.get_choice().to_string();
        let mut player_map: HashMap<String, String> = Default::default();
        player_map.insert(account_unique_id.to_string(), choice);

        let rockpaperscissors_repository = self.rockpaperscissors_repository.lock().await;

        let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;

        let response = rockpaperscissors_repository.insert_player_hashmap_for_wait(player_map).await;

        if response.is_ok() {
            return WaitHashmapResponse::new(true)
        }

        return WaitHashmapResponse::new(false)

    }

    async fn check_winner(&self, check_winner_request: CheckWinnerRequest) -> CheckWinnerResponse {
        println!("RockpaperscissorsServiceImpl: check_winner()");
        let account_unique_id=check_winner_request.get_account_unique_id();
        let opponent_id=check_winner_request.get_opponent_id();
        let rockpaperscissors_repository_guard = self.rockpaperscissors_repository.lock().await;
        let wait_hashmap_clone_mutex = rockpaperscissors_repository_guard.get_wait_hashmap();
        let wait_hashmap_clone_guard = wait_hashmap_clone_mutex.lock().await;


        let mut my_choice=wait_hashmap_clone_guard.get_player_hashmap(account_unique_id.to_string()).await.unwrap();
        let mut opponent_choice=wait_hashmap_clone_guard.get_player_hashmap(opponent_id.to_string()).await.unwrap();
        // if let Some(value) = wait_hashmap_clone_guard.get(&(account_unique_id.to_string())).await {
        //      my_choice = value.clone();
        // }
        // if let Some(value) = wait_hashmap_clone_guard.get(&(opponent_id.to_string())).await {
        //      opponent_choice = value.clone();
        // }

        let mut rng = rand::thread_rng();
        rng.gen::<bool>();
        let am_i_win = match (my_choice.as_str(), opponent_choice.as_str()) {
            ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => true,
            ("Scissors", "Rock") | ("Rock", "Paper") | ("Paper", "Scissors") => false,
            _ => rng.gen::<bool>(),// 기본값을 설정하거나 아무 작업을 하지 않음
        };





        drop(rockpaperscissors_repository_guard);

        CheckWinnerResponse::new(am_i_win)
    }
}


