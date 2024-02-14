use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquireUnitAttackPointResponse {
    attack_point: i32
}

impl AcquireUnitAttackPointResponse {
    pub fn new(attack_point: i32) -> Self {
        AcquireUnitAttackPointResponse { attack_point }
    }

    pub fn get_attack_point(&self) -> i32 {
        self.attack_point
    }
}
