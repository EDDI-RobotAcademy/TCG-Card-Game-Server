use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomWaitQueueRepository {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn dequeue_two_players_from_wait_queue(&self, count: usize) -> Vec<i32>;
}