use async_trait::async_trait;

#[async_trait]
pub trait NotifyPlayerActionRepository {
    async fn notify_to_opponent_what_you_do(&mut self, opponent_unique_id: i32, unit_card_number: i32) -> bool;
}