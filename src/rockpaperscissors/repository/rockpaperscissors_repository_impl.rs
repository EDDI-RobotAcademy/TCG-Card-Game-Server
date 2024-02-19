use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;



use crate::rockpaperscissors::entity::wait_queue::WaitQueue;

use crate::rockpaperscissors::repository::rockpaperscissors_repository::RockpaperscissorsRepository;



pub struct RockpaperscissorsRepositoryImpl {
    wait_queue: Arc<AsyncMutex<WaitQueue>>,
}

impl RockpaperscissorsRepositoryImpl {
    pub fn new() -> Self {
        RockpaperscissorsRepositoryImpl {
            wait_queue: Arc::new(AsyncMutex::new(WaitQueue::new())),
        }
    }

    pub fn get_wait_queue(&self) -> Arc<AsyncMutex<WaitQueue>> {
        Arc::clone(&self.wait_queue)
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
    async fn enqueue_player_tuple_for_wait(&self, player_tuple:(i32,String)) -> Result<bool, Box<dyn Error>> {
        println!("RockpaperscissorsRepositoryImpl: enqueue_player_tuple_for_wait()");
        let waiting_queue_guard = self.wait_queue.lock().await;
        waiting_queue_guard.enqueue_player_tuple(player_tuple).await;

        Ok(true)

    }

    async fn get_wait_queue_player_tuple_length(&self) -> i32 {
        println!("RockpaperscissorsRepositoryImpl: get_wait_queue_player_tuple_length()");
        let waiting_queue_guard = self.wait_queue.lock().await;
        let player_tuple_list_guard = waiting_queue_guard.player_tuple_list.lock().await;
        println!("length-->{:?}", player_tuple_list_guard.len());
        player_tuple_list_guard.len() as i32

    }
}

