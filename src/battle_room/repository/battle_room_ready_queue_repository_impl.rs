use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::entity::battle_room_ready_queue::BattleRoomReadyQueue;
use crate::battle_room::repository::battle_room_ready_queue_repository::BattleRoomReadyQueueRepository;

pub struct BattleRoomReadyQueueRepositoryImpl {
    ready_queue: BattleRoomReadyQueue,
}

impl BattleRoomReadyQueueRepositoryImpl {
    pub fn new() -> Self {
        BattleRoomReadyQueueRepositoryImpl {
            ready_queue: BattleRoomReadyQueue::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomReadyQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleRoomReadyQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomReadyQueueRepository for BattleRoomReadyQueueRepositoryImpl {
    async fn enqueue_player_id_for_ready(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        self.ready_queue.enqueue_player(account_unique_id).await;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use BattleRoomReadyQueueRepositoryImpl;

    #[tokio::test]
    async fn test_enqueue_player_id() {
        let repository = BattleRoomReadyQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let result = repository_arc
            .lock()
            .await
            .enqueue_player_id_for_ready(1)
            .await;

        assert!(result.is_ok());
    }
}
