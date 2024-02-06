use tokio::sync::mpsc;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_mpsc() {
        let (tx, mut rx) = mpsc::channel(32);

        if let Err(err) = tx.send("Hello, World!").await {
            eprintln!("Failed to send: {:?}", err);
        }

        if let Some(message) = rx.recv().await {
            println!("Received: {}", message);
        }
    }
}