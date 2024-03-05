use async_trait::async_trait;

#[async_trait]
pub trait MulliganRepository {
    async fn set_mulligan_timer(&self, account_unique_id: i32) -> bool;
    async fn check_opponent_mulligan_timer_over(&self, opponent_unique_id: i32) -> bool;
    async fn record_mulligan_finish(&self, account_unique_id: i32) -> bool;
    async fn check_opponent_mulligan_finish(&self, opponent_unique_id: i32) -> bool;
}