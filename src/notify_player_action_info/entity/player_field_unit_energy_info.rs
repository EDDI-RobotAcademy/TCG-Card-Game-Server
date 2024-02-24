use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::attached_energy_info::AttachedEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitEnergyInfo {
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl PlayerFieldUnitEnergyInfo {
    pub fn new(player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>) -> Self {
        PlayerFieldUnitEnergyInfo {
            player_field_unit_energy_map
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_data() {

        let mut attached_energy_map1 = HashMap::new();
        attached_energy_map1.insert(1, 3);
        attached_energy_map1.insert(2, 1);

        let mut attached_energy_map2 = HashMap::new();
        attached_energy_map2.insert(2, 1);
        attached_energy_map2.insert(3, 2);

        let attached_energy_info1 = AttachedEnergyInfo::new(attached_energy_map1);
        let attached_energy_info2 = AttachedEnergyInfo::new(attached_energy_map2);

        let mut field_unit_energy_map1 = HashMap::new();
        field_unit_energy_map1.insert(1, attached_energy_info1);
        let field_unit_energy_info1 = FieldUnitEnergyInfo::new(field_unit_energy_map1);

        let mut field_unit_energy_map2 = HashMap::new();
        field_unit_energy_map2.insert(3, attached_energy_info2);
        let field_unit_energy_info2 = FieldUnitEnergyInfo::new(field_unit_energy_map2);

        let mut player_field_unit_energy_map = HashMap::new();
        player_field_unit_energy_map.insert(PlayerIndex::Opponent, field_unit_energy_info1);
        player_field_unit_energy_map.insert(PlayerIndex::You, field_unit_energy_info2);

        let player_field_unit_energy_info = PlayerFieldUnitEnergyInfo::new(player_field_unit_energy_map);

        println!("{:?}", json!(player_field_unit_energy_info));
    }
}