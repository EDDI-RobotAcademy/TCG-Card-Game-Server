use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomWaitQueueRepository {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, diesel::result::Error>;
}