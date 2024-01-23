use uuid::Uuid;

#[derive(Debug)]
pub struct BattleRoom {
    pub id: Uuid,
    pub player_id_list: Vec<i32>
}

impl BattleRoom {
    pub fn new() -> BattleRoom {
        BattleRoom {
            id: Uuid::new_v4(),
            player_id_list: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player_id: i32) {
        self.player_id_list.push(player_id);
    }

    pub fn print_battle_room_status(&self) {
        println!("BattleRoom ID: {:?}", self.id);
        println!("Player IDs: {:?}", self.player_id_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_room() {
        let mut battle_room = BattleRoom::new();

        battle_room.add_player(1);
        battle_room.add_player(2);

        battle_room.print_battle_room_status();

        assert_eq!(battle_room.player_id_list, vec![1, 2]);
    }
}
