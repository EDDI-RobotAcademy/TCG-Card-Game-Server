use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::field_unit_info::FieldUnitInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

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