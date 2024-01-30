use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::timeout;

#[derive(Debug)]
enum Message {
    Enqueue(i32),
    Dequeue(tokio::sync::oneshot::Sender<Option<Vec<i32>>>),
}

struct WaitQueue {
    inner: Arc<Mutex<Vec<i32>>>,
    sender: mpsc::Sender<Message>,
}

impl WaitQueue {
    fn new() -> Self {
        let (sender, mut receiver) = mpsc::channel(100);

        let inner = Arc::new(Mutex::new(Vec::new()));
        let inner_clone = inner.clone();

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                match message {
                    Message::Enqueue(item) => {
                        let mut inner = inner_clone.lock().unwrap();
                        inner.push(item);
                    }
                    Message::Dequeue(sender) => {
                        let mut inner = inner_clone.lock().unwrap();
                        // Dequeue 2 items
                        let items: Vec<i32> = inner.drain(..2).collect();
                        let _ = sender.send(Some(items));
                    }
                }
            }
        });

        WaitQueue { inner, sender }
    }

    async fn enqueue(&self, item: i32) {
        self.sender.send(Message::Enqueue(item)).await.unwrap();
    }

    async fn dequeue(&self) -> Option<Vec<i32>> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender.send(Message::Dequeue(sender)).await.unwrap();
        receiver.await.unwrap()
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
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    dequeue_thread.abort();
}
