#[derive(Debug)]
pub struct NoticeApplyDamageToSpecificOpponentUnitRequest {
    opponent_unique_id: i32,
    opponent_unit_index: i32,
    damage: i32,
    updated_health_point: i32,
    dead_unit_index: i32,
}

impl NoticeApplyDamageToSpecificOpponentUnitRequest {
    pub fn new(opponent_unique_id: i32,
               opponent_unit_index: i32,
               damage: i32,
               updated_health_point: i32,
               dead_unit_index: i32) -> Self {
        NoticeApplyDamageToSpecificOpponentUnitRequest {
            opponent_unique_id,
            opponent_unit_index,
            damage,
            updated_health_point,
            dead_unit_index,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_opponent_unit_index(&self) -> i32 { self.opponent_unit_index }

    pub fn get_damage(&self) -> i32 { self.damage }

    pub fn get_updated_health_point(&self) -> i32 { self.updated_health_point }

    pub fn get_dead_unit_index(&self) -> i32 { self.dead_unit_index }
}