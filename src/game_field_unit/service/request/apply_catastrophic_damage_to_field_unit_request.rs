#[derive(Debug)]
pub struct ApplyCatastrophicDamageToFieldUnitRequest {
    opponent_unique_id: i32,
    catastrophic_damage: i32,
}

impl ApplyCatastrophicDamageToFieldUnitRequest {
    pub fn new(opponent_unique_id: i32, catastrophic_damage: i32) -> Self {
        ApplyCatastrophicDamageToFieldUnitRequest {
            opponent_unique_id,
            catastrophic_damage,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_catastrophic_damage(&self) -> i32 {
        self.catastrophic_damage
    }

}