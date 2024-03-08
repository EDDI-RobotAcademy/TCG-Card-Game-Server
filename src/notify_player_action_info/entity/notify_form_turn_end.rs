use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormTurnEnd {
    player_drawn_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    unit_index_turn_start_passive_list_map: HashMap<i32, Vec<i32>>,
}

impl NotifyFormTurnEnd {
    pub fn new(
        player_drawn_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
        player_field_energy_map: HashMap<PlayerIndex, i32>,
        player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
        unit_index_turn_start_passive_list_map: HashMap<i32, Vec<i32>>,
    ) -> Self {

        NotifyFormTurnEnd {
            player_drawn_card_list_map,
            player_field_energy_map,
            player_field_unit_health_point_map,
            player_field_unit_harmful_effect_map,
            player_field_unit_death_map,
            player_main_character_survival_map,
            unit_index_turn_start_passive_list_map
        }
    }
}