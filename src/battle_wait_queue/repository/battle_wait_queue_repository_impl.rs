use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_wait_queue::entity::battle_wait_queue::BattleWaitQueue;
use crate::battle_wait_queue::repository::battle_wait_queue_repository::BattleWaitQueueRepository;


pub struct BattleWaitQueueRepositoryImpl {
    battle_wait_queue: Arc<AsyncMutex<BattleWaitQueue>>,
}

impl BattleWaitQueueRepositoryImpl {
    pub fn new() -> Self {
        BattleWaitQueueRepositoryImpl {
            battle_wait_queue: Arc::new(AsyncMutex::new(BattleWaitQueue::new())),
        }
    }

    pub fn get_battle_wait_queue(&self) -> Arc<AsyncMutex<BattleWaitQueue>> {
        Arc::clone(&self.battle_wait_queue)
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleWaitQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleWaitQueueRepository for BattleWaitQueueRepositoryImpl {
    async fn enqueue_player_id_for_wait(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        println!("BattleWaitQueueRepositoryImpl: enqueue_player_id_for_wait()");
        let battle_wait_queue_guard = self.battle_wait_queue.lock().await;
        battle_wait_queue_guard.enqueue_player(account_unique_id).await;

        Ok(true)
    }

    async fn dequeue_two_players_from_wait_queue(&self, count: usize) -> Vec<i32> {
        println!("BattleWaitQueueRepositoryImpl: dequeue_two_players_from_wait_queue()");
        let battle_wait_queue_guard = self.battle_wait_queue.lock().await;
        battle_wait_queue_guard.dequeue_n_players(count).await
    }

    async fn dequeue_player_id_from_wait_queue(&self, account_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        println!("BattleWaitQueueRepositoryImpl: dequeue_player_id_from_wait_queue()");

        let battle_wait_queue_guard = self.battle_wait_queue.lock().await;
        let result= battle_wait_queue_guard.dequeue_player(account_unique_id).await;

        Ok(result)
    }

    async fn get_wait_queue_length(&self) -> i32 {
        println!("BattleWaitQueueRepositoryImpl: get_wait_queue_length()");
        let battle_wait_queue_guard = self.battle_wait_queue.lock().await;
        let player_id_list_guard = battle_wait_queue_guard.player_id_list.lock().await;
        player_id_list_guard.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_enqueue_player_id() {
        let repository = BattleWaitQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let result = repository_arc
            .lock()
            .await
            .enqueue_player_id_for_wait(1)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enqueue_and_dequeue_players() {
        let repository = BattleWaitQueueRepositoryImpl::new();
        let repository_arc = Arc::new(AsyncMutex::new(repository));

        let repository_guard = repository_arc.lock().await;
        repository_guard.enqueue_player_id_for_wait(1).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(2).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(3).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(4).await.expect("Can't enqueue");
        repository_guard.enqueue_player_id_for_wait(5).await.expect("Can't enqueue");

        let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
        let queue_before_dequeue = wait_queue_gaurd.player_id_list.lock().await;
        println!("Before dequeue: {:?}", queue_before_dequeue);
        drop(queue_before_dequeue);
        drop(wait_queue_gaurd);

        let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
        println!("dequeued_players: {:?}", dequeued_players);

        let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
        let queue_after_dequeue = wait_queue_gaurd.player_id_list.lock().await;
        println!("After dequeue: {:?}", queue_after_dequeue);
        drop(queue_after_dequeue);
        drop(wait_queue_gaurd);

        let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
        println!("dequeued_players: {:?}", dequeued_players);

        let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
        let queue_after_dequeue = wait_queue_gaurd.player_id_list.lock().await;
        println!("After dequeue: {:?}", queue_after_dequeue);
        drop(queue_after_dequeue);
        drop(wait_queue_gaurd);
    }
}
