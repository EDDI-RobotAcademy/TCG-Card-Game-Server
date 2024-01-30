use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository::BattleReadyMonitorRepository;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository_impl::BattleReadyMonitorRepositoryImpl;

use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::battle_room::repository::battle_room_wait_queue_repository::BattleRoomWaitQueueRepository;
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

    pub fn get_instance() -> Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleWaitQueueRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleWaitQueueRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    // // TODO: Thread가 배치된 위치가 다소 별로임 (Service 쪽에 배치 되는 것이 더 좋았음)
    // pub async fn start_dequeue_thread(&self) -> tokio::task::JoinHandle<()> {
    //     println!("start_dequeue_thread()");
    //     let wait_queue_clone = Arc::clone(&self.battle_wait_queue);
    //     tokio::spawn(async move {
    //         loop {
    //             let items = wait_queue_clone.lock().await.dequeue_n_players(2).await;
    //             println!("Dequeued from repository: {:?}", items);
    //
    //             if items.len() == 2 {
    //                 let battle_ready_monitor_repository_mutex = BattleReadyMonitorRepositoryImpl::get_instance();
    //                 let mut battle_ready_monitor_repository_guard = battle_ready_monitor_repository_mutex.lock().await;
    //
    //                 // TODO: Service에서 Repository 호출하도록 구성했어야함 (지금은 우선 진행)
    //                 battle_ready_monitor_repository_guard.save_battle_account_hash(items[0], BattleReadyStatus::SUCCESS).await;
    //                 battle_ready_monitor_repository_guard.save_battle_account_hash(items[1], BattleReadyStatus::SUCCESS).await;
    //                 drop(battle_ready_monitor_repository_guard);
    //
    //                 // TODO: 배틀룸으로 매칭된 사용자 이동 (위와 마찬가지)
    //                 let battle_room_repository_mutex = BattleRoomRepositoryImpl::get_instance();
    //                 let battle_room_repository_guard = battle_room_repository_mutex.lock().await;
    //
    //                 battle_room_repository_guard.set_player_to_battle_room(items).await.expect("전투 배치 실패");
    //                 drop(battle_room_repository_guard);
    //             }
    //
    //             tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    //         }
    //     })
    // }
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

    async fn get_wait_queue_length(&self) -> i32 {
        println!("BattleWaitQueueRepositoryImpl: dequeue_two_players_from_wait_queue()");
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

    // #[tokio::test]
    // async fn test_dequeue_thread() {
    //     let repository = Arc::new(BattleWaitQueueRepositoryImpl::new());
    //
    //     let dequeue_thread = repository.start_dequeue_thread().await;
    //
    //     let enqueue_threads: Vec<_> = (0..3).map(|i| {
    //         let repository = Arc::clone(&repository);
    //         tokio::spawn(async move {
    //             for j in 1..=5 {
    //                 repository.enqueue_player_id_for_wait(j + i * 5).await.unwrap();
    //                 println!("Enqueued: {}", j + i * 5);
    //                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //             }
    //         })
    //     }).collect();
    //
    //     tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
    //
    //     dequeue_thread.abort();
    // }

}
