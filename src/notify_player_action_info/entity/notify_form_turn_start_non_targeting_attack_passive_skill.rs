use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyFormTurnStartNonTargetingAttackPassiveSkill {
    player_field_unit_attack_map: HashMap<PlayerIndex, FieldUnitAttackInfo>,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl NotifyFormTurnStartNonTargetingAttackPassiveSkill {
    pub fn new(
        player_field_unit_attack_map: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> Self {

        NotifyFormTurnStartNonTargetingAttackPassiveSkill {
            player_field_unit_attack_map,
            player_field_unit_health_point_map,
            player_field_unit_harmful_effect_map,
            player_field_unit_death_map
        }
    }
}