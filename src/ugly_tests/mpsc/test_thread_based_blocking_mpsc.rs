use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::sync::mpsc::Receiver;
use tokio::task;

async fn produce_messages(tx: mpsc::Sender<String>) {
    for i in 0..10 {
        let message = format!("Message {}", i);
        tx.send(message).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}

async fn consume_messages_inner(id: usize, rx: Arc<Mutex<mpsc::Receiver<String>>>) {
    loop {
        let mut rx = rx.lock().unwrap();
        match rx.recv().await {
            Some(message) => {
                println!("Consumer {} received: {}", id, message);
            }
            None => break,
        }
    }
}

async fn consume_messages(id: usize, rx: Arc<Mutex<mpsc::Receiver<String>>>) {
    task::spawn_blocking(move || tokio::runtime::Runtime::new().unwrap().block_on(consume_messages_inner(id, rx)));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use tokio::{task, test};
    use futures::future::try_join_all;

    #[test]
    async fn test_blocking_mpsc() {
        let (tx, rx) = mpsc::channel::<String>(32);

        let rx = Arc::new(Mutex::new(rx));

        let consumers: Vec<_> = (0..4)
            .map(|i| {
                let rx = Arc::clone(&rx);
                task::spawn(async move {
                    consume_messages(i, rx).await;
                })
            })
            .collect();

        let producer = task::spawn(async move {
            produce_messages(tx).await;
        });

        tokio::try_join!(producer, async {
            try_join_all(consumers).await?;
            Ok(())
        })
        .unwrap();
    }
}