use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct FirstTurnDecisionWaitQueue {
    pub player_id_list: Mutex<Vec<i32>>,
    pub player_choice_list:Mutex<Vec<String>>
}

impl FirstTurnDecisionWaitQueue {
    pub fn new() -> FirstTurnDecisionWaitQueue {
        FirstTurnDecisionWaitQueue {
            player_id_list: Mutex::new(Vec::new()),
            player_choice_list: Mutex::new(Vec::new())
        }
    }

    pub async fn enqueue_player(&self, player_id: i32) {
        let mut guard = self.player_id_list.lock().await;
        guard.push(player_id);

        println!("player_id_list: {:?}", guard);
    }
    pub async fn enqueue_player_choice(&self, player_choice: String) {
        let mut guard = self.player_choice_list.lock().await;
        guard.push(player_choice);

        println!("player_choice_list: {:?}", guard);
    }

    pub async fn dequeue_player(&self) -> Option<i32> {
        let mut guard = self.player_id_list.lock().await;
        guard.pop()
    }

    pub async fn process_queue(&self, max_players: usize) {
        let mut guard = self.player_id_list.lock().await;
        while guard.len() > max_players {
            guard.remove(0);
        }
    }

    pub async fn dequeue_n_players(&self, count: usize) -> Vec<i32> {
        let mut guard = self.player_id_list.lock().await;
        let mut dequeued_players = Vec::new();

        if guard.len() >= count {
            dequeued_players.push(guard.pop().unwrap());
            println!("dequeued_players: {:?}", dequeued_players);
            dequeued_players.push(guard.pop().unwrap());
            println!("dequeued_players: {:?}", dequeued_players);
        }

        dequeued_players
    }
    pub async fn dequeue_n_players_choice(&self, count: usize) -> Vec<String> {
        let mut guard = self.player_choice_list.lock().await;
        let mut dequeued_players_choice = Vec::new();

        if guard.len() >= count {
            dequeued_players_choice.push(guard.pop().unwrap());
            println!("dequeued_players_choice: {:?}", dequeued_players_choice);
            dequeued_players_choice.push(guard.pop().unwrap());
            println!("dequeued_players_choice: {:?}", dequeued_players_choice);

        }

        dequeued_players_choice
    }
}











