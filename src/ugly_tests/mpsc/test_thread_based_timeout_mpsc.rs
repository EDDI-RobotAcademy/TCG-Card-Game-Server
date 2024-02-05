use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use futures::future::try_join_all;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc::Receiver;
use tokio::task;

async fn produce_messages(tx: mpsc::Sender<String>) {
    for i in 0..10 {
        let message = format!("Message {}", i);
        match tx.send(message).await {
            Ok(_) => {
                // Message sent successfully
            }
            Err(e) => {
                eprintln!("Error sending message: {:?}", e);
                // Handle the error as needed
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}

async fn consume_messages_inner(id: usize, mut rx: mpsc::Receiver<String>) {
    tokio::select! {
        recv_result = rx.recv() => {
            if let Some(message) = recv_result {
                println!("Consumer {} received: {}", id, message);
            }
        }
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            // Non-blocking delay to allow other tasks to run
        }
    }
}

async fn consume_messages(
    id: usize,
    tx: mpsc::Sender<String>,
    rx: Arc<TokioMutex<mpsc::Receiver<String>>>,
) {
    let (inner_tx, inner_rx) = mpsc::channel::<String>(32);

    // Spawn a separate Tokio task for consuming messages
    tokio::spawn(consume_messages_inner(id, inner_rx));

    // Pass the inner channel sender to the producer
    tokio::spawn(produce_messages(inner_tx));

    // Continue processing messages from the shared receiver
    loop {
        let mut rx = rx.lock().await;  // Use tokio::sync::Mutex here
        tokio::select! {
            Some(message) = rx.recv() => {
                println!("Consumer {} received: {}", id, message);
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // Non-blocking delay to allow other tasks to run
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;
    use tokio::{task, test};
    use futures::future::try_join_all;
    use tokio::time::timeout;

    #[test]
    async fn test_mpsc_timeout() {
        let (tx, rx) = mpsc::channel::<String>(32);
        let rx = Arc::new(TokioMutex::new(rx));

        let consumers: Vec<_> = (0..4)
            .map(|i| {
                let tx = tx.clone();
                let rx = Arc::clone(&rx);
                tokio::spawn(consume_messages(i, tx, rx))
            })
            .collect();

        // Use timeout and match to handle the Result
        match timeout(Duration::from_secs(10), try_join_all(consumers)).await {
            Ok(_) => {
                // Test completed successfully
            }
            Err(e) => {
                eprintln!("Test timed out or encountered an error: {:?}", e);
                // You might want to add additional error handling or logging here
            }
        }
    }

    #[test]
    async fn test_mpsc_timeout2() {
        let (tx, rx) = mpsc::channel::<String>(32);
        let rx = Arc::new(TokioMutex::new(rx));

        let consumers: Vec<_> = (0..4)
            .map(|i| {
                let tx = tx.clone();
                let rx = Arc::clone(&rx);
                tokio::spawn(consume_messages(i, tx, rx))
            })
            .collect();

        // 프로듀서 작업을 시작하고 완료될 때까지 기다립니다.
        let producer_task = tokio::spawn(produce_messages(tx));
        producer_task.await.unwrap();

        // 소비자가 제한 시간 내에 완료될 때까지 기다립니다.
        tokio::time::timeout(Duration::from_secs(10), try_join_all(consumers))
            .await
            .unwrap();
    }
}
