use std::collections::HashMap;
use tokio::sync::Mutex;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;

#[derive(Debug,)]
pub struct RockPaperScissorsWaitHash {
    player_choice_hash: Mutex<HashMap<i32, String>>
}

impl RockPaperScissorsWaitHash {
    pub fn new() -> RockPaperScissorsWaitHash {
        RockPaperScissorsWaitHash {
            player_choice_hash: Mutex::new(HashMap::new())
        }
    }

    pub async fn save_choice(&mut self, account_unique_id: i32, choice: String) {
        println!("RockPaperScissorsWaitHash: save_choice");

        let mut guard = self.player_choice_hash.lock().await;
        guard.insert(account_unique_id, choice);

        println!("player_hashmap_list: {:?}", guard);
    }

    pub async fn get_choice(&self, account_unique_id: i32) -> Option<String> {
        println!("RockPaperScissorsWaitHash: get_choice");

        let guard = self.player_choice_hash.lock().await;
        for (key, value) in guard.iter() {
            if key == &account_unique_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                return Some(value.clone());
            }
        }

        // 찾지 못했을 때는 None 을 반환
        None
    }

    pub async fn remove_choice(&mut self, account_unique_id: i32) {
        println!("RockPaperScissorsWaitHash: remove_choice");

        let mut guard = self.player_choice_hash.lock().await;
        guard.retain(|key, _| key != &account_unique_id);

        println!("player_hashmap_list: {:?}", guard);
    }

    pub async fn change_draw_choices(&self, account_unique_id: i32, opponent_unique_id: i32) {
        println!("RockPaperScissorsWaitHash: change_draw_choices");

        let mut guard = self.player_choice_hash.lock().await;
        let mut my_choice = None;
        let mut opponent_choice = None;

        for (key, value) in guard.iter() {
            if key == &account_unique_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                my_choice = Some(value.clone());
            }
            if key == &opponent_unique_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                opponent_choice = Some(value.clone());
            }
        }

        if my_choice.clone() == opponent_choice.clone() {
            println!("비긴 결과에 대해 랜덤 choice 를 부여합니다.");
            let choices = vec!["Rock", "Paper", "Scissors"];
            let mut rng = StdRng::from_entropy(); // 시드 값을 현재 시간 등의 엔트로피로 설정

            // "Rock", "Paper", "Scissors" 중에서 중복되지 않게 2개 선택
            let random_choices: Vec<&str> = choices
                .choose_multiple(&mut rng, 2)
                .cloned()
                .collect();

            guard.insert(account_unique_id, random_choices[0].to_string());
            guard.insert(opponent_unique_id, random_choices[1].to_string());

            println!("player_hashmap_list: {:?}", guard);
        }
    }

    pub async fn check_opponent_choice(&self, opponent_unique_id: i32) -> Option<bool> {
        println!("RockPaperScissorsWaitHash: check_opponent_choice");

        let mut guard = self.player_choice_hash.lock().await;

        for (key, value) in guard.iter() {
            if key == &opponent_unique_id && value != "Dummy" {
                return Some(true);
            }
        }

        Some(false)
    }
}