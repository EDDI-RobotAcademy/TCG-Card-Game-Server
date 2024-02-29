use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormUseUnitEnergyBoostSupportCard {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NotifyFormUseUnitEnergyBoostSupportCard {
    pub fn new(player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_deck_card_use_list_map: HashMap<PlayerIndex, Vec<i32>>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {

        NotifyFormUseUnitEnergyBoostSupportCard {
            player_hand_use_map,
            player_deck_card_use_list_map,
            player_field_unit_energy_map
        }
    }
}