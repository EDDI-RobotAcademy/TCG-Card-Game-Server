use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

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
    pub async fn change_draw_choice_hashmap(&self, account_unique_id: String,opponent_id: String,random_choice:Vec<&str>) -> Option<String> {
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
        if  (Some(my_choice.clone()), Some(opponent_choice.clone())) != (None, None)
        {
            if my_choice==opponent_choice
            {

                guard.insert(account_unique_id,random_choice[0].to_string());
                guard.insert(opponent_id,random_choice[1].to_string());
            }
        }

        // 찾지 못했을 때는 None을 반환
        None
    }


    // pub async fn dequeue_player_tuple(&self) -> Option<(i32,String)> {
    //     let mut guard = self.player_tuple_list.lock().await;
    //     guard.pop()
    // }

    // pub async fn process_queue(&self, max_players: usize) {
    //     let mut guard = self.player_tuple_list.lock().await;
    //     while guard.len() > max_players {
    //         guard.remove(0);
    //     }
    // }
    //
    // pub async fn dequeue_n_players_tuple(&self, count: usize) -> Vec<(i32,String)> {
    //     let mut guard = self.player_tuple_list.lock().await;
    //     let mut dequeued_players = Vec::new();
    //
    //     if guard.len() >= count {
    //         dequeued_players.push(guard.pop().unwrap());
    //         println!("dequeued_players: {:?}", dequeued_players);
    //         dequeued_players.push(guard.pop().unwrap());
    //         println!("dequeued_players: {:?}", dequeued_players);
    //     }
    //
    //     dequeued_players
    // }

}











