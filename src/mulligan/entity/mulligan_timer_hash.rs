use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

#[derive(Debug,)]
pub struct MulliganTimerHash {
    timer_map: Mutex<HashMap<i32, Instant>>
}

impl MulliganTimerHash {
    pub fn new() -> MulliganTimerHash {
        MulliganTimerHash {
            timer_map: Mutex::new(HashMap::new())
        }
    }

    pub async fn start(&mut self, account_unique_id: i32) {
        println!("MulliganTimerHash: start");

        let mut timer_map_guard = self.timer_map.lock().await;
        timer_map_guard.insert(account_unique_id, Instant::now());
    }

    pub async fn check(&self, account_unique_id: i32, duration: Duration) -> Option<bool> {
        println!("MulliganTimerHash: check");

        let check_time = Instant::now();
        let mut timer_map_guard = self.timer_map.lock().await;
        for (key, value) in timer_map_guard.iter() {
            if key == &account_unique_id && (check_time - *value) > duration {
                println!("Mulligan Time limit over for account {}", account_unique_id);
                return Some(false);
            }
        }

        Some(true)
    }

    pub async fn remove(&mut self, account_unique_id: i32) {
        println!("MulliganTimerHash: remove");

        let mut timer_map_guard = self.timer_map.lock().await;
        timer_map_guard.retain(|key, _| key != &account_unique_id);
    }
}