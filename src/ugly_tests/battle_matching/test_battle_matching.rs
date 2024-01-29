use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct TestBattleRoomWaitingQueue {
    sender: mpsc::Sender<i32>,
    receiver: Mutex<mpsc::Receiver<i32>>,
}

impl TestBattleRoomWaitingQueue {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        TestBattleRoomWaitingQueue {
            sender,
            receiver: Mutex::new(receiver),
        }
    }

    pub async fn enqueue_player(&self, player_id: i32) {
        self.sender.send(player_id).await.expect("Failed to enqueue player");
    }

    pub async fn is_ready(&self, count: usize) -> bool {
        let mut collected = Vec::new();

        for _ in 0..count {
            if let Some(player_id) = self.receiver.lock().await.recv().await {
                collected.push(player_id);
            } else {
                break;
            }
        }

        collected.len() >= count
    }

    pub async fn dequeue_n_players(&self, count: usize) -> Vec<i32> {
        let mut dequeued_players = Vec::new();

        for _ in 0..count {
            if let Some(player_id) = self.receiver.lock().await.recv().await {
                dequeued_players.push(player_id);
            } else {
                break;
            }
        }

        println!("dequeued_players: {:?}", dequeued_players);
        dequeued_players
    }
}

async fn monitor_queue(mut queue: Arc<TestBattleRoomWaitingQueue>) {
    loop {
        sleep(Duration::from_millis(500)).await;

        if queue.is_ready(2).await {
            let dequeued_players = queue.dequeue_n_players(2).await;
            println!("Monitoring: Dequeued players: {:?}", dequeued_players);
        } else {
            println!("Monitoring: Not enough players ready.");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::ugly_tests::battle_matching::test_battle_matching::{monitor_queue, TestBattleRoomWaitingQueue};

    #[tokio::test]
    async fn test_battle_matching() {
        let queue = Arc::new(TestBattleRoomWaitingQueue::new());

        let queue_clone_1 = Arc::clone(&queue);
        sleep(Duration::from_millis(500)).await;
        tokio::spawn(async move {
            let mut count = 0;
            loop {
                count += 1;
                println!("current count: {}", count);
                queue_clone_1.enqueue_player(count).await;

                sleep(Duration::from_millis(500)).await;
            }
        });

        let queue_clone_2 = Arc::clone(&queue);
        tokio::spawn(async move {
            loop {
                queue_clone_2.enqueue_player(2).await;

                sleep(Duration::from_millis(500)).await;
            }
        });

        let queue_clone_monitor = Arc::clone(&queue);
        tokio::spawn(monitor_queue(queue_clone_monitor));

        sleep(Duration::from_secs(10)).await;
    }

}