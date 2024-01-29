use tokio::sync::Mutex;
use std::sync::Arc;

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

        println!("player_id_list: {:?}", guard);
    }

    pub async fn dequeue_player(&self) -> Option<i32> {
        let mut guard = self.player_id_list.lock().await;
        guard.pop()
    }

    pub async fn process_queue(&self, max_players: usize) {
        let mut guard = self.player_id_list.lock().await;
        while guard.len() > max_players {
            guard.remove(0);
        }
    }

    pub async fn dequeue_n_players(&self, count: usize) -> Vec<i32> {
        let mut guard = self.player_id_list.lock().await;
        let mut dequeued_players = Vec::new();

        if guard.len() >= count {
            dequeued_players.push(guard.pop().unwrap());
            dequeued_players.push(guard.pop().unwrap());
        }
        println!("dequeued_players: {:?}", dequeued_players);

        dequeued_players
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

        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(1).await;
        }).await.unwrap();

        let waiting_queue_clone = Arc::clone(&waiting_queue);
        tokio::spawn(async move {
            waiting_queue_clone.enqueue_player(2).await;
        }).await.unwrap();

        waiting_queue.process_queue(2).await;

        let guard = waiting_queue.player_id_list.lock().await;
        assert_eq!(guard.len(), 2);
        assert_eq!(*guard, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_process_queue_removes_excess_players() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

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

        waiting_queue.process_queue(1).await;

        let guard = waiting_queue.player_id_list.lock().await;
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0], 2);
    }

    #[tokio::test]
    async fn test_dequeue_player() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

        waiting_queue.enqueue_player(1).await;

        let dequeued_player = waiting_queue.dequeue_player().await;
        assert_eq!(dequeued_player, Some(1));
    }

    #[tokio::test]
    async fn test_dequeue_players() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

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

        let mut dequeued_players = waiting_queue.dequeue_n_players(2).await;

        dequeued_players.sort();

        assert_eq!(dequeued_players, vec![1, 2]);

        let guard = waiting_queue.player_id_list.lock().await;
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn test_dequeue_players_single_player() {
        let waiting_queue = Arc::new(BattleRoomWaitingQueue::new());

        waiting_queue.enqueue_player(1).await;

        let dequeued_players = waiting_queue.dequeue_n_players(2).await;

        assert!(dequeued_players.is_empty());

        let guard = waiting_queue.player_id_list.lock().await;
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0], 1);
    }
}
