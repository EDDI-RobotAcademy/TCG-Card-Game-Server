use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct BattleRoom {
    pub id: i32,
    pub player_id_list: Vec<i32>,
}

lazy_static! {
    // Mutex for synchronizing access to the room counter
    static ref ROOM_COUNTER: Mutex<i32> = Mutex::new(0);
}

impl BattleRoom {
    pub fn new() -> BattleRoom {
        // Increment the room counter for each new room
        let mut counter = ROOM_COUNTER.lock().unwrap();
        *counter += 1;

        BattleRoom {
            id: *counter,
            player_id_list: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player_id: i32) {
        self.player_id_list.push(player_id);
    }

    pub fn print_battle_room_status(&self) {
        println!("BattleRoom ID: {}", self.id);
        println!("Player IDs: {:?}", self.player_id_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_room() {
        let mut battle_room1 = BattleRoom::new();
        let mut battle_room2 = BattleRoom::new();

        battle_room1.add_player(1);
        battle_room1.add_player(2);

        battle_room2.add_player(3);
        battle_room2.add_player(4);

        battle_room1.print_battle_room_status();
        battle_room2.print_battle_room_status();

        assert_eq!(battle_room1.player_id_list, vec![1, 2]);
        assert_eq!(battle_room2.player_id_list, vec![3, 4]);

        assert_ne!(battle_room1.id, battle_room2.id);
    }
}
