use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormUseCatastrophicDamageItemCard {
    player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
    player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl NotifyFormUseCatastrophicDamageItemCard {
    pub fn new(player_hand_use_map: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
               player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
               player_deck_card_lost_list_map: HashMap<PlayerIndex, Vec<i32>>,) -> Self {

        NotifyFormUseCatastrophicDamageItemCard {
            player_hand_use_map,
            player_field_unit_health_point_map,
            player_field_unit_death_map,
            player_main_character_health_point_map,
            player_main_character_survival_map,
            player_deck_card_lost_list_map
        }
    }
}