use std::collections::HashMap;
use tokio::sync::Mutex;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult;

#[derive(Debug,)]
pub struct RockPaperScissorsResultHash {
    result_hash: Mutex<HashMap<i32, RockPaperScissorsResult>>
}

impl RockPaperScissorsResultHash {
    pub fn new() -> RockPaperScissorsResultHash {
        RockPaperScissorsResultHash {
            result_hash: Mutex::new(HashMap::new())
        }
    }

    pub async fn save_result(&mut self, account_unique_id: i32, result: RockPaperScissorsResult) {
        println!("RockPaperScissorsResultHash: save_result");

        let mut guard = self.result_hash.lock().await;
        guard.insert(account_unique_id, result);

        println!("result_hash: {:?}", guard);
    }

    pub async fn get_result(&self, account_unique_id: i32) -> Option<RockPaperScissorsResult> {
        println!("RockPaperScissorsResultHash: get_result");

        let guard = self.result_hash.lock().await;
        for (key, value) in guard.iter() {
            if key == &account_unique_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                return Some(value.clone());
            }
        }

        // 찾지 못했을 때는 None 을 반환
        None
    }

    pub async fn remove_result(&mut self, account_unique_id: i32) {
        println!("RockPaperScissorsResultHash: remove_result");

        let mut guard = self.result_hash.lock().await;
        guard.retain(|key, _| key != &account_unique_id);

        println!("result_hash: {:?}", guard);
    }
}