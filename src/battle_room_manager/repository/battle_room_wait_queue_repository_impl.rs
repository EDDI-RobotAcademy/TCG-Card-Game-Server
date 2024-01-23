use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room_manager::entity::battle_room_ready_queue::BattleRoomReadyQueue;
use crate::battle_room_manager::entity::battle_room_wait_queue::BattleRoomWaitingQueue;
use crate::battle_room_manager::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
use crate::receiver::entity::receive_data::ReceiveData;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;


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
    async fn enqueue_player_id(&self, account_unique_id: i32) -> Result<bool, diesel::result::Error> {
        self.wait_queue.enqueue_player(account_unique_id).await;

        Ok(true)
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
            .enqueue_player_id(1)
            .await;

        assert!(result.is_ok());
    }
}
