use std::collections::HashMap;
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;

pub struct BattleReadyAccountHash {
    battle_ready_account_states: HashMap<i32, BattleReadyStatus>,
}

impl BattleReadyAccountHash {
    pub(crate) fn new() -> Self {
        Self {
            battle_ready_account_states: HashMap::new(),
        }
    }

    pub(crate) fn set_user_ready_state(&mut self, user_id: i32, state: BattleReadyStatus) {
        self.battle_ready_account_states.insert(user_id, state);
    }

    pub fn get_user_ready_state(&self, user_id: i32) -> Option<&BattleReadyStatus> {
        self.battle_ready_account_states.get(&user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_manager() {
        let mut battle_room_ready_hash = BattleReadyAccountHash::new();

        battle_room_ready_hash.set_user_ready_state(1, BattleReadyStatus::WAIT);
        battle_room_ready_hash.set_user_ready_state(2, BattleReadyStatus::SUCCESS);

        assert_eq!(battle_room_ready_hash.get_user_ready_state(1), Some(&BattleReadyStatus::WAIT));
        assert_eq!(battle_room_ready_hash.get_user_ready_state(2), Some(&BattleReadyStatus::SUCCESS));
        assert_eq!(battle_room_ready_hash.get_user_ready_state(3), None);
    }
}