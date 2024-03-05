use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::rock_paper_scissors::entity::rock_paper_scissors_waiting_timer::RockPaperScissorsWaitingTimer;
use crate::rock_paper_scissors_waiting_timer::repository::rock_paper_scissors_waiting_timer_repository::RockPaperScissorsWaitingTimerRepository;


pub struct RockPaperScissorsWaitingTimerRepositoryImpl {
    rock_paper_scissors_waiting_timer: RockPaperScissorsWaitingTimer,
}

impl RockPaperScissorsWaitingTimerRepositoryImpl {
    pub fn new() -> Self {
        RockPaperScissorsWaitingTimerRepositoryImpl {
            rock_paper_scissors_waiting_timer: RockPaperScissorsWaitingTimer::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockPaperScissorsWaitingTimerRepositoryImpl>> =
                Arc::new(AsyncMutex::new(RockPaperScissorsWaitingTimerRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl RockPaperScissorsWaitingTimerRepository for RockPaperScissorsWaitingTimerRepositoryImpl {
    async fn set_rock_paper_scissors_waiting_timer(&mut self, account_unique_id: i32) {
        println!("MatchWaitingTimerRepositoryImpl: set_match_waiting_timer()");

        self.rock_paper_scissors_waiting_timer.start_timer(account_unique_id);
    }

    async fn check_rock_paper_scissors_waiting_timer_expired(&mut self, account_unique_id: i32, duration: Duration) -> bool {
        println!("MatchWaitingTimerRepositoryImpl: check_match_waiting_timer_expired()");

        self.rock_paper_scissors_waiting_timer.check_timer(account_unique_id, duration)
    }
}