#[derive(Debug)]
pub struct NoticeInstantDeathOfSpecificUnitRequest {
    opponent_unique_id: i32,
    dead_unit_index: i32,
}

impl NoticeInstantDeathOfSpecificUnitRequest {
    pub fn new(opponent_unique_id: i32,
               dead_unit_index: i32,) -> Self {
        NoticeInstantDeathOfSpecificUnitRequest {
            opponent_unique_id,
            dead_unit_index,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_dead_unit_index(&self) -> i32 { self.dead_unit_index }
}