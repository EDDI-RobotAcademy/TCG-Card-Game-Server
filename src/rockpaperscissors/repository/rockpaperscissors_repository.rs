use std::collections::HashMap;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RockpaperscissorsRepository {
    async fn insert_player_hashmap_for_wait(&self, hashmap: HashMap<String, String>) -> Result<bool, Box<dyn Error>>;

}