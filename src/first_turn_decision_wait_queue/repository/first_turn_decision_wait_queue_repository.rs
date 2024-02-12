use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait FirstTurnDecisionWaitQueueRepository {
    async fn enqueue_player_id_for_wait(&self, player_unique_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn enqueue_player_choice_for_wait(&self, player_choice: String) -> Result<bool, Box<dyn Error>>;
    async fn dequeue_two_players_id_from_wait_queue(&self, count: usize) -> Vec<i32>;
    async fn dequeue_two_players_choice_from_wait_queue(&self, count: usize) -> Vec<String>;
    async fn get_wait_queue_player_id_length(&self) -> i32;
    async fn get_wait_queue_player_choice_length(&self) -> i32;
}