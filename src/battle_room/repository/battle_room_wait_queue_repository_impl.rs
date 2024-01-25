use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::entity::battle_room_wait_queue::BattleRoomWaitingQueue;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;


pub struct BattleRoomWaitQueueRepositoryImpl {
    wait_queue: BattleRoomWaitingQueue,
    // ready_queue: BattleRoomReadyQueue,
    // receiver_battle_room_manager_channel_arc: Option<Arc<ReceiverTransmitterLegacyChannel>>,
    // battle_room_manager_transmitter_channel_arc: Option<Arc<ReceiverTransmitterLegacyChannel>>
}

impl BattleRoomWaitQueueRepositoryImpl {
    pub fn new() -> Self {
        BattleRoomWaitQueueRepositoryImpl {
            wait_queue: BattleRoomWaitingQueue::new(),
            // ready_queue: BattleRoomReadyQueue::new(),
            // acceptor_receiver_channel_arc: None,
            // receiver_transmitter_channel_arc: None
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomWaitQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleRoomWaitQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomWaitQueueRepository for BattleRoomWaitQueueRepositoryImpl {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        self.wait_queue.enqueue_player(account_unique_id).await;

        Ok(true)
    }

    async fn dequeue_two_players_from_wait_queue(&self, count: usize) -> Vec<i32> {
        self.wait_queue.dequeue_n_players(count).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_enqueue_player_id() {
        let repository = BattleRoomWaitQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let result = repository_arc
            .lock()
            .await
            .enqueue_player_id_for_wait(1)
            .await;

        assert!(result.is_ok());
    }
}
