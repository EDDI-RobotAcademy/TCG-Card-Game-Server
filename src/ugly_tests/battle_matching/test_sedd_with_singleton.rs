use std::collections::VecDeque;
use std::sync::Arc;
use tokio::time::timeout;

use tokio::sync::{Mutex, mpsc, oneshot};

// Entity: WaitQueue
pub struct WaitQueue {
    inner: VecDeque<i32>,
}

impl WaitQueue {
    pub fn new() -> Self {
        Self { inner: VecDeque::new() }
    }

    pub fn enqueue(&mut self, item: i32) {
        self.inner.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<Vec<i32>> {
        if self.inner.len() >= 2 {
            Some(self.inner.drain(..2).collect())
        } else {
            None
        }
    }
}

// Repository: WaitQueueRepository
pub struct WaitQueueRepository {
    sender: mpsc::Sender<Message>,
}

enum Message {
    Enqueue(i32, oneshot::Sender<()>),
    Dequeue(oneshot::Sender<Option<Vec<i32>>>),
}

impl WaitQueueRepository {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut wait_queue = WaitQueue::new();

            while let Some(msg) = receiver.recv().await {
                match msg {
                    Message::Enqueue(item, sender) => {
                        wait_queue.enqueue(item);
                        sender.send(()).unwrap();
                    }
                    Message::Dequeue(sender) => {
                        let items = wait_queue.dequeue();
                        sender.send(items).unwrap();
                    }
                }
            }
        });

        Self { sender }
    }

    pub async fn enqueue(&self, item: i32) {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Message::Enqueue(item, sender)).await.unwrap();
        receiver.await.unwrap();
    }

    pub async fn dequeue(&self) -> Option<Vec<i32>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Message::Dequeue(sender)).await.unwrap();
        receiver.await.unwrap()
    }
}

#[tokio::test]
async fn test_enqueue_and_dequeue() {
    let wait_queue_repo = Arc::new(WaitQueueRepository::new());

    let enqueue_threads: Vec<_> = (0..3).map(|i| {
        let wait_queue_repo = Arc::clone(&wait_queue_repo);
        tokio::spawn(async move {
            for j in 1..=5 {
                wait_queue_repo.enqueue(j + i * 5).await;
                println!("Enqueued: {}", j + i * 5);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        })
    })
        .collect::<Vec<tokio::task::JoinHandle<()>>>();

    let dequeue_thread = tokio::spawn(async move {
        loop {
            if let Some(items) = wait_queue_repo.dequeue().await {
                println!("Dequeued: {:?}", items);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    let join_handles: Vec<_> = enqueue_threads.into_iter().collect();

    tokio::select! {
        _ = async {
            for handle in join_handles {
                handle.await.expect("Error in enqueue thread");
            }
        } => {},
        _ = timeout(tokio::time::Duration::from_secs(15), tokio::time::sleep(tokio::time::Duration::from_secs(0))) => {},
    };

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    dequeue_thread.abort();
}
