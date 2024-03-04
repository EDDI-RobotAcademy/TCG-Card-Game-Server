use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait ActionWaitingTimerRepository {
    async fn set_action_waiting_timer(&mut self, account_unique_id: i32, duration: Duration) -> bool;
}