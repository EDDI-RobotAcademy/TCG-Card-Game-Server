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

    pub async fn check(&self, account_unique_id: i32) -> Option<bool> {
        println!("MulliganStatusHash: get_done");

        let mut list_guard = self.list.lock().await;
        if list_guard.contains(&account_unique_id) {

            return Some(true);
        }

        Some(false)
    }

    pub async fn remove(&self, account_unique_id: i32) {
        let mut list_guard = self.list.lock().await;
        list_guard.retain(|v| v != &account_unique_id);
    }
}