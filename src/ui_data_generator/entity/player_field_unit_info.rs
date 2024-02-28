use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_info::FieldUnitInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitInfo {
    player_field_unit_map: HashMap<PlayerIndex, HashMap<i32, FieldUnitInfo>>,
}

impl PlayerFieldUnitInfo {
    pub fn new(player_field_unit_map: HashMap<PlayerIndex, HashMap<i32, FieldUnitInfo>>) -> Self {
        PlayerFieldUnitInfo {
            player_field_unit_map
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {
        todo!()
    }
}