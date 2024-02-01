use std::collections::HashMap;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;

#[derive(Debug)]
pub struct BattleReadyAccountHash {
    battle_ready_account_hash_status: HashMap<i32, BattleReadyAccountHashStatus>,
}

impl BattleReadyAccountHash {
    pub(crate) fn new() -> Self {
        Self {
            battle_ready_account_hash_status: HashMap::new(),
        }
    }

    pub(crate) fn set_user_ready_state(&mut self, user_id: i32, state: BattleReadyAccountHashStatus) {
        self.battle_ready_account_hash_status.insert(user_id, state);
    }

    pub fn get_user_ready_state(&self, user_id: i32) -> Option<&BattleReadyAccountHashStatus> {
        self.battle_ready_account_hash_status.get(&user_id)
    }

    pub fn get_status_map(&self) -> &HashMap<i32, BattleReadyAccountHashStatus> {
        &self.battle_ready_account_hash_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_manager() {
        let mut battle_room_ready_hash = BattleReadyAccountHash::new();

        battle_room_ready_hash.set_user_ready_state(1, BattleReadyAccountHashStatus::WAIT);
        battle_room_ready_hash.set_user_ready_state(2, BattleReadyAccountHashStatus::SUCCESS);

        assert_eq!(battle_room_ready_hash.get_user_ready_state(1), Some(&BattleReadyAccountHashStatus::WAIT));
        assert_eq!(battle_room_ready_hash.get_user_ready_state(2), Some(&BattleReadyAccountHashStatus::SUCCESS));
        assert_eq!(battle_room_ready_hash.get_user_ready_state(3), None);
    }
}