#[derive(Debug)]
pub struct AcquireUnitHarmfulStatusEffectRequest {
    opponent_unique_id: i32,
    opponent_unit_index: i32,
}

impl AcquireUnitHarmfulStatusEffectRequest {
    pub fn new(opponent_unique_id: i32, opponent_unit_index: i32) -> Self {
        AcquireUnitHarmfulStatusEffectRequest {
            opponent_unique_id,
            opponent_unit_index,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_opponent_unit_index(&self) -> i32 {
        self.opponent_unit_index
    }
}