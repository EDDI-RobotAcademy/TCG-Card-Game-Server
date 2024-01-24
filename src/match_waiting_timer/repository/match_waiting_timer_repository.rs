use async_trait::async_trait;

#[async_trait]
pub trait MatchWaitingTimerRepository {
    async fn set_match_waiting_timer(&mut self, account_unique_id: i32);
}