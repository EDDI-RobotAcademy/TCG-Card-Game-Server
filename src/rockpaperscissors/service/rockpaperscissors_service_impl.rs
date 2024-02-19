use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use tokio::time::sleep;
use std::time::{Duration, Instant};
use crate::match_waiting_timer::entity::match_waiting_timer::MatchWaitingTimer;

use crate::rockpaperscissors::service::rockpaperscissors_service::RockpaperscissorsService;
use crate::rockpaperscissors::service::request::wait_queue_request::WaitQueueRequest;
use crate::rockpaperscissors::service::response::wait_queue_response::WaitQueueResponse;
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


    async fn enqueue_player_tuple_to_wait_queue(&self, wait_queue_request: WaitQueueRequest) -> WaitQueueResponse {
        println!("RockpaperscissorsServiceImpl: enqueue_player_tuple_to_wait_queue()");

        let account_unique_id = self.parse_account_unique_id(wait_queue_request.get_session_id()).await;
        let choice=wait_queue_request.get_choice().to_string();
        let mut player_tuple: (i32, String)=(account_unique_id,choice);

        let rockpaperscissors_repository = self.rockpaperscissors_repository.lock().await;

        let mut match_waiting_timer_repository = self.match_waiting_timer_repository.lock().await;
        match_waiting_timer_repository.set_match_waiting_timer(account_unique_id).await;

        let response = rockpaperscissors_repository.enqueue_player_tuple_for_wait(player_tuple).await;

        if response.is_ok() {
            return WaitQueueResponse::new(true)
        }

        return WaitQueueResponse::new(false)

    }

    async fn check_winner(&self, check_winner_request: CheckWinnerRequest) -> CheckWinnerResponse {
        println!("RockpaperscissorsServiceImpl: check_winner()");
        let account_id=check_winner_request.get_account_id();
        let rockpaperscissors_repository_guard = self.rockpaperscissors_repository.lock().await;
        let wait_queue_clone_mutex = rockpaperscissors_repository_guard.get_wait_queue();
        let wait_queue_clone_guard = wait_queue_clone_mutex.lock().await;

        let mut players_data = wait_queue_clone_guard.dequeue_player_tuple().await;
        let player1=players_data.unwrap();
        let mut players_data = wait_queue_clone_guard.dequeue_player_tuple().await;
        let player2=players_data.unwrap();


        let (winner_account_unique_id, am_i_win, result_is_draw) = match (player1.1.as_str(),
                                                                          player2.1.as_str(), )
        {
            ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => {
                if account_id == player1.0{
                    (player1.0, true, false)
                } else {
                    (player1.0, false, false)
                }
                // 플레이어 1이 이길 때
            }
            ("Scissors", "Rock") | ("Rock", "Paper") | ("Paper", "Scissors") => {
                if account_id == player2.0 {
                    (player2.0, true, false)
                } else {
                    (player2.0, false, false)
                }
                // 플레이어 2가 이길 때
            }
            _ => {
                // 그 외의 경우 (무승부)
                println!("무승부");
                (0, false, true)
            }
        };
        (winner_account_unique_id, am_i_win, result_is_draw);
        wait_queue_clone_guard.enqueue_player_tuple(player1).await;
        wait_queue_clone_guard.enqueue_player_tuple(player2).await;


        drop(rockpaperscissors_repository_guard);

        CheckWinnerResponse::new(am_i_win,result_is_draw)
    }
}