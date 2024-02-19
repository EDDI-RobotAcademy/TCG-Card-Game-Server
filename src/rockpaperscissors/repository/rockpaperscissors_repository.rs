use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RockpaperscissorsRepository {
    async fn enqueue_player_tuple_for_wait(&self, player_tuple: (i32,String)) -> Result<bool, Box<dyn Error>>;
    // async fn dequeue_two_players_id_from_wait_queue(&self, count: usize) -> Vec<i32>;
    // async fn dequeue_two_players_choice_from_wait_queue(&self, count: usize) -> Vec<String>;
    async fn get_wait_queue_player_tuple_length(&self) -> i32;
    // async fn get_wait_queue_player_choice_length(&self) -> i32;
}