use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomReadyQueueRepository {
    async fn enqueue_player_id_for_ready(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>>;
}