use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;



use crate::rockpaperscissors::entity::wait_hashmap::WaitHashMap;

use crate::rockpaperscissors::repository::rockpaperscissors_repository::RockpaperscissorsRepository;



pub struct RockpaperscissorsRepositoryImpl {
    wait_hashmap: Arc<AsyncMutex<WaitHashMap>>,
}

impl RockpaperscissorsRepositoryImpl {
    pub fn new() -> Self {
        RockpaperscissorsRepositoryImpl {
            wait_hashmap: Arc::new(AsyncMutex::new(WaitHashMap::new())),
        }
    }

    pub fn get_wait_hashmap(&self) -> Arc<AsyncMutex<WaitHashMap>> {
        Arc::clone(&self.wait_hashmap)
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockpaperscissorsRepositoryImpl>> =
                Arc::new(AsyncMutex::new(RockpaperscissorsRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl RockpaperscissorsRepository for RockpaperscissorsRepositoryImpl {
    async fn insert_player_hashmap_for_wait(&self, player_hashmap:HashMap<String,String>) -> Result<bool, Box<dyn Error>> {
        println!("RockpaperscissorsRepositoryImpl: insert_player_hashmap_for_wait()");
        let waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.insert_player_hashmap(player_hashmap).await;

        Ok(true)

    }

    async fn  change_draw_choice_repo(&self, account_unique_id: String, opponent_id: String, random_choice:Vec<&str>) -> Result<bool, Box<dyn Error>> {
        println!("RockpaperscissorsRepositoryImpl: change_draw_choice()");
        let waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.change_draw_choice_hashmap(account_unique_id,opponent_id,random_choice);
        Ok(true)
    }


    // async fn get_wait_queue_player_tuple_length(&self) -> i32 {
    //     println!("RockpaperscissorsRepositoryImpl: get_wait_queue_player_tuple_length()");
    //     let waiting_queue_guard = self.wait_queue.lock().await;
    //     let player_tuple_list_guard = waiting_queue_guard.player_tuple_list.lock().await;
    //     println!("length-->{:?}", player_tuple_list_guard.len());
    //     player_tuple_list_guard.len() as i32
    //
    // }
}

