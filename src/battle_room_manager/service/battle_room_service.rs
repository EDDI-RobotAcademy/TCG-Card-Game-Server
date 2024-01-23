use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomService {
    async fn enqueue_player_id_to_wait_queue(&self, account_unique_id: i32) -> Result<bool, diesel::result::Error>;
}