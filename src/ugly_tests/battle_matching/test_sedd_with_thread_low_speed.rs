use std::collections::VecDeque;
use std::sync::Arc;
use tokio::time::timeout;

use tokio::sync::{Mutex, mpsc, oneshot};

pub struct WaitQueue {
    sender: mpsc::Sender<Message>,
}

enum Message {
    Enqueue(i32, oneshot::Sender<()>),
    Dequeue(oneshot::Sender<Vec<i32>>),
}

impl WaitQueue {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut inner = Vec::new();

            while let Some(msg) = receiver.recv().await {
                match msg {
                    Message::Enqueue(item, sender) => {
                        inner.push(item);
                        sender.send(()).unwrap();
                    }
                    Message::Dequeue(sender) => {
                        if inner.len() >= 2 {
                            let items: Vec<i32> = inner.drain(..2.min(inner.len())).collect();
                            sender.send(items).unwrap();
                        } else {
                            sender.send(Vec::new()).unwrap();
                        }
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
        receiver.await.ok()
    }
}

#[tokio::test]
async fn test_enqueue_and_dequeue() {
    let wait_queue = Arc::new(WaitQueue::new());

    let enqueue_threads: Vec<_> = (0..3).map(|i| {
        let wait_queue = Arc::clone(&wait_queue);
        tokio::spawn(async move {
            for j in 1..=5 {
                wait_queue.enqueue(j + i * 5).await;
                println!("Enqueued: {}", j + i * 5);
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        })
    })
        .collect::<Vec<tokio::task::JoinHandle<()>>>();

    let dequeue_thread = tokio::spawn(async move {
        loop {
            if let Some(items) = wait_queue.dequeue().await {
                println!("Dequeued: {:?}", items);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
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
