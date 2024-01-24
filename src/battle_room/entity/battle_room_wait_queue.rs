use tokio::sync::Mutex;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct BattleRoomWaitingQueue {
    pub player_id_list: Mutex<Vec<i32>>,
}

impl BattleRoomWaitingQueue {
    pub fn new() -> BattleRoomWaitingQueue {
        BattleRoomWaitingQueue {
            player_id_list: Mutex::new(Vec::new()),
        }
    }

    pub async fn enqueue_player(&self, player_id: i32) {
        let mut guard = self.player_id_list.lock().await;
        guard.push(player_id);
    }

    pub async fn process_queue(&self, max_players: usize) {
        let mut guard = self.player_id_list.lock().await;
        while guard.len() > max_players {
            guard.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_enqueue_and_process_queue() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

        // 여러 프로듀서가 enqueue
        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(1).await;
        }).await.unwrap();

        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(2).await;
        }).await.unwrap();

        // 비동기적으로 대기열 처리
        waiting_queue.process_queue(2).await;

        // 대기열 확인
        let guard = waiting_queue.player_id_list.lock().await;
        assert_eq!(guard.len(), 2);
        assert_eq!(*guard, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_process_queue_removes_excess_players() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

        // 여러 프로듀서가 enqueue
        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(1).await;
        })
        .await
        .unwrap();

        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(2).await;
        })
        .await
        .unwrap();

        // 대기열 처리
        waiting_queue.process_queue(1).await;

        // 대기열 확인 (빈자리가 없어서 맨 앞의 플레이어가 제거되어야 함)
        let guard = waiting_queue.player_id_list.lock().await;
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0], 2);
    }
}
