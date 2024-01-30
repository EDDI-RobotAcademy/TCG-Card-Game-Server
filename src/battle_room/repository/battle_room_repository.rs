use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BattleRoomRepository {
    async fn set_players_to_battle_room(&self, account_unique_id_list: Vec<i32>) -> Result<bool, Box<dyn Error>>;
}