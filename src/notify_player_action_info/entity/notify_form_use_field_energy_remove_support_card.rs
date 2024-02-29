use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormUseFieldEnergyRemoveSupportCard {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
}

impl NotifyFormUseFieldEnergyRemoveSupportCard {
    pub fn new(player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_field_energy_map: HashMap<PlayerIndex, i32>,) -> Self {

        NotifyFormUseFieldEnergyRemoveSupportCard {
            player_hand_use_map,
            player_field_energy_map,
        }
    }
}