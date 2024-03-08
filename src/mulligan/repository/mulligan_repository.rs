use async_trait::async_trait;

#[async_trait]
pub trait MulliganRepository {
    async fn set_mulligan_timer(&self, account_unique_id: i32) -> bool;
    async fn check_mulligan_timer_over(&self, account_unique_id: i32) -> bool;
    async fn remove_mulligan_timer(&self, account_unique_id: i32) -> bool;
    async fn record_mulligan_finish(&self, account_unique_id: i32) -> bool;
    async fn check_mulligan_finish(&self, account_unique_id: i32) -> bool;
    async fn remove_mulligan_finish_record(&self, account_unique_id: i32) -> bool;
}