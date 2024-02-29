use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormUseFieldEnergyToUnit {
    player_field_energy_map: HashMap<PlayerIndex, i32>,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NotifyFormUseFieldEnergyToUnit {
    pub fn new(player_field_energy_map: HashMap<PlayerIndex, i32>,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,) -> Self {

        NotifyFormUseFieldEnergyToUnit {
            player_field_energy_map,
            player_field_unit_energy_map
        }
    }
}