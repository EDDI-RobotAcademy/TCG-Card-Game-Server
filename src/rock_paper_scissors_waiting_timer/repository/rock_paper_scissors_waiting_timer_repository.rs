use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait RockPaperScissorsWaitingTimerRepository {
    async fn set_rock_paper_scissors_waiting_timer(&mut self, account_unique_id: i32);
    async fn check_rock_paper_scissors_waiting_timer_expired(&mut self, account_unique_id: i32, duration: Duration) -> bool;
}