use tokio::sync::Mutex;

#[derive(Debug,)]
pub struct MulliganFinishedPlayerList {
    list: Mutex<Vec<i32>>
}

impl MulliganFinishedPlayerList {
    pub fn new() -> MulliganFinishedPlayerList {
        MulliganFinishedPlayerList {
            list: Mutex::new(Vec::new())
        }
    }

    pub async fn set(&mut self, account_unique_id: i32) {
        println!("MulliganStatusHash: set_done");

        let mut list_guard = self.list.lock().await;
        list_guard.push(account_unique_id);
    }

    pub async fn check(&self, opponent_unique_id: i32) -> Option<bool> {
        println!("MulliganStatusHash: get_done");

        let mut list_guard = self.list.lock().await;
        if list_guard.contains(&opponent_unique_id) {
            list_guard.retain(|v| v != &opponent_unique_id);
            return Some(true);
        }

        Some(false)
    }
}