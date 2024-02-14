use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUnitToGameFieldResponse {
    placed_unit_index: i32
}

impl AddUnitToGameFieldResponse {
    pub fn new(placed_unit_index: i32) -> Self {
        AddUnitToGameFieldResponse { placed_unit_index }
    }

    pub fn get_placed_unit_index(&self) -> i32 {
        self.placed_unit_index
    }
}
