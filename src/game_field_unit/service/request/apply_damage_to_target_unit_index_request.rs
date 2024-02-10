use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;

#[derive(Debug)]
pub struct ApplyDamageToTargetUnitIndexRequest {
    opponent_unique_id: i32,
    opponent_target_unit_index: i32,
    damage: i32,
}

impl ApplyDamageToTargetUnitIndexRequest {
    pub fn new(opponent_unique_id: i32, opponent_target_unit_index: i32, damage: i32) -> Self {
        ApplyDamageToTargetUnitIndexRequest {
            opponent_unique_id,
            opponent_target_unit_index,
            damage
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_opponent_target_unit_index(&self) -> i32 {
        self.opponent_target_unit_index
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }
}
