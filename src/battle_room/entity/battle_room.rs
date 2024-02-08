use lazy_static::lazy_static;
use std::sync::Mutex;


#[derive(Debug, Clone)]
pub enum RoomStatus {
    STATUS_FREE,
    STATUS_FULL,
}

// TODO: 추후 다 대 다 전투를 진행하는 상황에 대해 고려 할 필요가 있음 (이건 1 대 1)
#[derive(Debug, Clone)]
pub struct BattleRoom {
    pub id: i32,
    pub player_id_list: Vec<i32>,
    pub status: RoomStatus,
}

lazy_static! {
    static ref ROOM_COUNTER: Mutex<i32> = Mutex::new(0);
}

static mut ROOM_COUNT: i32 = 0;
static MAX_ROOM_SIZE: usize = 2;

impl BattleRoom {
    pub fn new() -> BattleRoom {
        let mut counter = ROOM_COUNTER.lock().unwrap();
        *counter += 1;

        BattleRoom {
            id: *counter,
            player_id_list: Vec::new(),
            status: RoomStatus::STATUS_FREE,
        }
    }

    pub fn add_player(&mut self, player_id: i32) {
        if !self.is_full() {
            self.player_id_list.push(player_id);
            if self.is_full() {
                self.status = RoomStatus::STATUS_FULL;
            }
        } else {
            println!("BattleRoom {} is full. Cannot add more players.", self.id);
        }
    }

    pub fn is_full(&self) -> bool {
        self.player_id_list.len() >= MAX_ROOM_SIZE
    }

    pub fn print_battle_room_status(&self) {
        println!("BattleRoom ID: {}", self.id);
        println!("Player IDs: {:?}", self.player_id_list);
    }

    pub fn get_room_count() -> i32 {
        *ROOM_COUNTER.lock().unwrap()
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

    #[test]
    fn test_battle_room_create() {
        let mut battle_room1 = BattleRoom::new();
        let mut battle_room2 = BattleRoom::new();

        battle_room1.add_player(1);
        battle_room1.add_player(2);
        battle_room1.add_player(3); // Trying to add more than the room size

        battle_room2.add_player(4);
        battle_room2.add_player(5);

        battle_room1.print_battle_room_status();
        battle_room2.print_battle_room_status();

        assert_eq!(battle_room1.player_id_list, vec![1, 2]);
        assert_eq!(battle_room2.player_id_list, vec![4, 5]);

        assert_ne!(battle_room1.id, battle_room2.id);

        let room_count = BattleRoom::get_room_count();
        assert_eq!(room_count, 2);
    }
}
