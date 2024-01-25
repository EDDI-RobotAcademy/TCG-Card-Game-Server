use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait MatchWaitingTimerRepository {
    async fn set_match_waiting_timer(&mut self, account_unique_id: i32);
    async fn check_match_waiting_timer_expired(&mut self, account_unique_id: i32, duration: Duration) -> bool;
}