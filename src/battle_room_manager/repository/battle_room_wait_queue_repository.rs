use async_trait::async_trait;


#[async_trait]
pub trait BattleRoomWaitQueueRepository {
    async fn enqueue_player_id(&self, account_unique_id: &str) -> Result<bool, diesel::result::Error>;
}