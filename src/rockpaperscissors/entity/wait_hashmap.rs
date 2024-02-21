use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;

#[derive(Debug,)]
pub struct WaitHashMap {

    player_hashmap_list: Mutex<HashMap<String, String>>
}

impl WaitHashMap {
    pub fn new() -> WaitHashMap {
        WaitHashMap {

            player_hashmap_list: Mutex::new(HashMap::new())
        }
    }



    pub async fn insert_player_hashmap(&self, player_hashmap: HashMap<String,String>) {
        let mut guard = self.player_hashmap_list.lock().await;
        for (key, value) in player_hashmap.iter() {
            guard.insert(key.clone(), value.clone());
        }

        println!("player_hashmap_list-->: {:?}", guard);
    }
    pub async fn get_player_hashmap(&self, account_id: String) -> Option<String> {
        let guard = self.player_hashmap_list.lock().await;

        for (key, value) in guard.iter() {
            if key == &account_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                return Some(value.clone());
            }
        }

        // 찾지 못했을 때는 None을 반환
        None
    }
    pub async fn change_draw_choice_hashmap(&self, account_unique_id: String,opponent_id: String) -> Option<String> {
        println!("same_choice_occur->insert_random_choice");
        let mut guard = self.player_hashmap_list.lock().await;
        let mut my_choice=None;
        let mut opponent_choice=None;
        for (key, value) in guard.iter() {
            if key == &account_unique_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                my_choice=Some(value.clone());
            }
            if key == &opponent_id {
                // 특정 키에 대한 값을 찾았을 때, 해당 값을 반환
                opponent_choice=Some(value.clone());
            }
        }

            if my_choice.clone()==opponent_choice.clone()
            {
                let choices = vec!["Rock", "Paper", "Scissors"];
                let mut rng = StdRng::from_entropy(); // 시드 값을 현재 시간 등의 엔트로피로 설정

                // "Rock", "Paper", "Scissors" 중에서 중복되지 않게 2개 선택
                let random_choices: Vec<&str> = choices
                    .choose_multiple(&mut rng, 2)
                    .cloned()
                    .collect();
                guard.insert(account_unique_id,random_choices[0].to_string());

                guard.insert(opponent_id,random_choices[1].to_string());

                println!("hashmap_list----->>>{:?}",guard);
            }


        // 찾지 못했을 때는 None을 반환
        None
    }

}











