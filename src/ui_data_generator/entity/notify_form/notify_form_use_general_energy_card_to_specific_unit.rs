use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormUseGeneralEnergyCardToSpecificUnit {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NotifyFormUseGeneralEnergyCardToSpecificUnit {
    pub fn new(
        player_index: PlayerIndex,
        used_hand_card_info: UsedHandCardInfo,
        field_unit_energy_info: FieldUnitEnergyInfo) -> Self {

        let mut player_hand_use_map = HashMap::new();
        player_hand_use_map.insert(player_index.clone(), used_hand_card_info);

        let mut player_field_unit_energy_map = HashMap::new();
        player_field_unit_energy_map.insert(player_index.clone(), field_unit_energy_info);

        NotifyFormUseGeneralEnergyCardToSpecificUnit {
            player_hand_use_map,
            player_field_unit_energy_map
        }
    }
}