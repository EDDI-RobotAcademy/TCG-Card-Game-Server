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











