use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTargetingAttackPassiveSkillResponseForm {
    is_success: bool,
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    index_list_of_passive_skill_to_handle: Vec<i32>
}

impl DeployTargetingAttackPassiveSkillResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
               player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
               index_list_of_passive_skill_to_handle: Vec<i32>
    ) -> Self {
        DeployTargetingAttackPassiveSkillResponseForm {
            is_success,
            player_field_unit_health_point_map,
            player_field_unit_harmful_effect_map,
            player_field_unit_death_map,
            index_list_of_passive_skill_to_handle
        }
    }

    pub fn default() -> DeployTargetingAttackPassiveSkillResponseForm {

        DeployTargetingAttackPassiveSkillResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            Vec::new())
    }
}