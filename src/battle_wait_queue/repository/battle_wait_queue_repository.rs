use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BattleWaitQueueRepository {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn dequeue_two_players_from_wait_queue(&self, count: usize) -> Vec<i32>;
    async fn dequeue_player_id_from_wait_queue(&self,account_unique_id:i32) -> Result<bool, Box<dyn Error>>;
    async fn get_wait_queue_length(&self) -> i32;
}