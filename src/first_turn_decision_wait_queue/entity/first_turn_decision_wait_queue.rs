use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct FirstTurnDecisionWaitQueue {

    pub player_tuple_list:Mutex<Vec<(i32,String)>>
}

impl FirstTurnDecisionWaitQueue {
    pub fn new() -> FirstTurnDecisionWaitQueue {
        FirstTurnDecisionWaitQueue {

            player_tuple_list: Mutex::new(Vec::new())
        }
    }


    pub async fn enqueue_player_tuple(&self, player_tuple: (i32,String)) {
        let mut guard = self.player_tuple_list.lock().await;
        guard.push(player_tuple);

        println!("어카운트 유니크 아이디와 가위바위보 타입 잘 들어갔니?-->: {:?}", guard);
    }


    pub async fn dequeue_player_tuple(&self) -> Option<(i32,String)> {
        let mut guard = self.player_tuple_list.lock().await;
        guard.pop()
    }

    pub async fn process_queue(&self, max_players: usize) {
        let mut guard = self.player_tuple_list.lock().await;
        while guard.len() > max_players {
            guard.remove(0);
        }
    }

    pub async fn dequeue_n_players_tuple(&self, count: usize) -> Vec<(i32,String)> {
        let mut guard = self.player_tuple_list.lock().await;
        let mut dequeued_players = Vec::new();

        if guard.len() >= count {
            dequeued_players.push(guard.pop().unwrap());
            println!("dequeued_players: {:?}", dequeued_players);
            dequeued_players.push(guard.pop().unwrap());
            println!("dequeued_players: {:?}", dequeued_players);
        }

        dequeued_players
    }

}











