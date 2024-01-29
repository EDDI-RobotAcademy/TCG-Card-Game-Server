use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Message {
    Enqueue(i32),
    Dequeue(tokio::sync::oneshot::Sender<Option<i32>>),
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
                        let item = inner.pop();
                        let _ = sender.send(item);
                    }
                }
            }
        });

        WaitQueue { inner, sender }
    }

    async fn enqueue(&self, item: i32) {
        self.sender.send(Message::Enqueue(item)).await.unwrap();
    }

    async fn dequeue(&self) -> Option<i32> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender.send(Message::Dequeue(sender)).await.unwrap();
        receiver.await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::task::JoinHandle;
    use tokio::time::{sleep, timeout};
    use crate::ugly_tests::battle_matching::test_multi_thread_queue::WaitQueue;

    #[tokio::test]
    async fn test_enqueue_and_dequeue() {
        let wait_queue = Arc::new(WaitQueue::new());

        let enqueue_threads: Vec<_> = (0..3).map(|i| {
            let wait_queue = Arc::clone(&wait_queue);
            tokio::spawn(async move {
                for j in 1..=5 {
                    wait_queue.enqueue(j + i * 5).await;
                    println!("Enqueued: {}", j + i * 5);
                    sleep(Duration::from_secs(1)).await;
                }
            })
        })
            .collect::<Vec<JoinHandle<_>>>();

        let dequeue_thread = tokio::spawn(async move {
            loop {
                if let Some(item) = wait_queue.dequeue().await {
                    println!("Dequeued: {}", item);
                }
                sleep(Duration::from_secs(2)).await;
            }
        });

        let join_handles: Vec<_> = enqueue_threads.into_iter().collect();

        tokio::select! {
        _ = async {
            for handle in join_handles {
                handle.await.expect("Error in enqueue thread");
            }
        } => {},
        _ = timeout(Duration::from_secs(15), tokio::time::sleep(Duration::from_secs(0))) => {},
    };

        sleep(Duration::from_secs(5)).await;

        dequeue_thread.abort();
    }
}
