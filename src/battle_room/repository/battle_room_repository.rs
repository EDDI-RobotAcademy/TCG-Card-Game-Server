use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomRepository {
    async fn set_players_to_battle_room(&self, account_unique_id_list: Vec<i32>) -> Result<bool, Box<dyn Error>>;
    async fn remove_battle_room_player(&mut self, account_unique_id: i32) -> bool;
    async fn what_is_the_room_number(&self, account_unique_id: i32) -> Option<i32>;
    async fn find_opponent_unique_id(&self, account_unique_id: i32) -> Option<i32>;
}