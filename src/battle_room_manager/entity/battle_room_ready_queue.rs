use tokio::sync::Mutex;
use std::sync::Arc;

pub struct BattleRoomReadyQueue {
    pub player_id_list: Mutex<Vec<i32>>,
}

impl BattleRoomReadyQueue {
    pub fn new() -> BattleRoomReadyQueue {
        BattleRoomReadyQueue {
            player_id_list: Mutex::new(Vec::new()),
        }
    }

    pub async fn enqueue_player(&self, player_id: i32) {
        let mut guard = self.player_id_list.lock().await;
        guard.push(player_id);
    }

    pub async fn dequeue_player(&self) -> Option<i32> {
        let mut guard = self.player_id_list.lock().await;
        guard.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_enqueue_and_dequeue_players() {
        let ready_queue = Arc::new(BattleRoomReadyQueue::new());

        let ready_queue_clone = Arc::clone(&ready_queue);
        tokio::spawn(async move {
            ready_queue_clone.enqueue_player(1).await;
        })
        .await
        .unwrap();

        let ready_queue_clone = Arc::clone(&ready_queue);
        tokio::spawn(async move {
            ready_queue_clone.enqueue_player(2).await;
        })
        .await
        .unwrap();

        let player1 = ready_queue.dequeue_player().await;
        let player2 = ready_queue.dequeue_player().await;

        assert_eq!(player1, Some(2));
        assert_eq!(player2, Some(1));
    }
}
