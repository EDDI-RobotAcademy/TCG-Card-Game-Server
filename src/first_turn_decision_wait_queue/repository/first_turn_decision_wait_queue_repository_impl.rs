use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;


use crate::first_turn_decision_wait_queue::entity::first_turn_decision_wait_queue::FirstTurnDecisionWaitQueue;

use crate::first_turn_decision_wait_queue::repository::first_turn_decision_wait_queue_repository::FirstTurnDecisionWaitQueueRepository;


pub struct FirstTurnDecisionWaitQueueRepositoryImpl {
    first_turn_decision_wait_queue: Arc<AsyncMutex<FirstTurnDecisionWaitQueue>>,
}

impl FirstTurnDecisionWaitQueueRepositoryImpl {
    pub fn new() -> Self {
        FirstTurnDecisionWaitQueueRepositoryImpl {
            first_turn_decision_wait_queue: Arc::new(AsyncMutex::new(FirstTurnDecisionWaitQueue::new())),
        }
    }

    pub fn get_first_turn_decision_wait_queue(&self) -> Arc<AsyncMutex<FirstTurnDecisionWaitQueue>> {
        Arc::clone(&self.first_turn_decision_wait_queue)
    }

    pub fn get_instance() -> Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FirstTurnDecisionWaitQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(FirstTurnDecisionWaitQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl FirstTurnDecisionWaitQueueRepository for FirstTurnDecisionWaitQueueRepositoryImpl {
    async fn enqueue_player_id_for_wait(&self, player_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: enqueue_player_id_for_wait()");
        let first_turn_decision_player_id_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        first_turn_decision_player_id_wait_queue_guard.enqueue_player(player_unique_id).await;

        Ok(true)
    }
    async fn enqueue_player_choice_for_wait(&self, player_choice: String) -> Result<bool, Box<dyn Error>> {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: enqueue_player_choice_for_wait()");
        let first_turn_decision_player_choice_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        first_turn_decision_player_choice_wait_queue_guard.enqueue_player_choice(player_choice).await;

        Ok(true)
    }

    async fn dequeue_two_players_id_from_wait_queue(&self, count: usize) -> Vec<i32> {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: dequeue_two_players_id_from_wait_queue()");
        let first_turn_decision_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        first_turn_decision_wait_queue_guard.dequeue_n_players(count).await
    }
    async fn dequeue_two_players_choice_from_wait_queue(&self, count: usize) -> Vec<String> {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: dequeue_two_players_choice_from_wait_queue()");
        let first_turn_decision_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        first_turn_decision_wait_queue_guard.dequeue_n_players_choice(count).await
    }

    async fn get_wait_queue_player_id_length(&self) -> i32 {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: get_wait_queue_player_id_length()");
        let first_turn_decision_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        let player_id_list_guard = first_turn_decision_wait_queue_guard.player_id_list.lock().await;
        player_id_list_guard.len() as i32
    }
    async fn get_wait_queue_player_choice_length(&self) -> i32 {
        println!("FirstTurnDecisionWaitQueueRepositoryImpl: get_wait_queue_player_choice_length()");
        let first_turn_decision_wait_queue_guard = self.first_turn_decision_wait_queue.lock().await;
        let player_choice_list_guard = first_turn_decision_wait_queue_guard.player_choice_list.lock().await;
        player_choice_list_guard.len() as i32
    }

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::test;
//     use tokio::time::timeout;
//
//     #[tokio::test]
//     async fn test_enqueue_player_id() {
//         let repository = BattleWaitQueueRepositoryImpl::new();
//         let repository_arc = Arc::new(AsyncMutex::new(repository));
//
//         let result = repository_arc
//             .lock()
//             .await
//             .enqueue_player_id_for_wait(1)
//             .await;
//
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_enqueue_and_dequeue_players() {
//         let repository = BattleWaitQueueRepositoryImpl::new();
//         let repository_arc = Arc::new(AsyncMutex::new(repository));
//
//         let repository_guard = repository_arc.lock().await;
//         repository_guard.enqueue_player_id_for_wait(1).await.expect("Can't enqueue");
//         repository_guard.enqueue_player_id_for_wait(2).await.expect("Can't enqueue");
//         repository_guard.enqueue_player_id_for_wait(3).await.expect("Can't enqueue");
//         repository_guard.enqueue_player_id_for_wait(4).await.expect("Can't enqueue");
//         repository_guard.enqueue_player_id_for_wait(5).await.expect("Can't enqueue");
//
//         let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
//         let queue_before_dequeue = wait_queue_gaurd.player_id_list.lock().await;
//         println!("Before dequeue: {:?}", queue_before_dequeue);
//         drop(queue_before_dequeue);
//         drop(wait_queue_gaurd);
//
//         let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
//         println!("dequeued_players: {:?}", dequeued_players);
//
//         let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
//         let queue_after_dequeue = wait_queue_gaurd.player_id_list.lock().await;
//         println!("After dequeue: {:?}", queue_after_dequeue);
//         drop(queue_after_dequeue);
//         drop(wait_queue_gaurd);
//
//         let dequeued_players = repository_guard.dequeue_two_players_from_wait_queue(2).await;
//         println!("dequeued_players: {:?}", dequeued_players);
//
//         let wait_queue_gaurd = repository_guard.battle_wait_queue.lock().await;
//         let queue_after_dequeue = wait_queue_gaurd.player_id_list.lock().await;
//         println!("After dequeue: {:?}", queue_after_dequeue);
//         drop(queue_after_dequeue);
//         drop(wait_queue_gaurd);
//     }
// }
