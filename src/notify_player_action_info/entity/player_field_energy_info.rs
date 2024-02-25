use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldEnergyInfo {
    player_field_energy_map: HashMap<PlayerIndex, i32>,
}

impl PlayerFieldEnergyInfo {
    pub fn new(player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        PlayerFieldEnergyInfo {
            player_field_energy_map
        }
    }

    pub fn get_player_field_energy_map(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_field_energy_map
    }
}