use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentHandToFieldUnitActionResponse {
    hand_to_field_unit: i32,
}

impl NotifyOpponentHandToFieldUnitActionResponse {
    pub fn new(hand_to_field_unit: i32) -> Self {
        NotifyOpponentHandToFieldUnitActionResponse { hand_to_field_unit }
    }

    pub fn hand_to_field_unit(&self) -> i32 {
        self.hand_to_field_unit
    }
}
