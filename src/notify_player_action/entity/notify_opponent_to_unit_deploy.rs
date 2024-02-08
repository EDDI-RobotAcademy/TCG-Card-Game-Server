use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToUnitDeploy {
    unit_card_id: i32,
}

impl NotifyOpponentToUnitDeploy {
    pub fn new(unit_card_id: i32) -> Self {
        Self { unit_card_id }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
