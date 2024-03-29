use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::attached_energy_info::AttachedEnergyInfo;
use crate::ui_data_generator::entity::extra_effect_info::ExtraEffectInfo;
use crate::ui_data_generator::entity::harmful_effect_info::HarmfulStatusInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldUnitInfo {
    attached_energy: AttachedEnergyInfo,
    extra_effect: ExtraEffectInfo,
    harmful_status: HarmfulStatusInfo,
    attack_point: i32,
    health_point: i32,
    is_alive: bool,
}

impl FieldUnitInfo {
    pub fn new(attached_energy: AttachedEnergyInfo,
               extra_effect: ExtraEffectInfo,
               harmful_status: HarmfulStatusInfo,
               attack_point: i32,
               health_point: i32,
               is_alive: bool,) -> Self {
        FieldUnitInfo {
            attached_energy,
            extra_effect,
            harmful_status,
            attack_point,
            health_point,
            is_alive
        }
    }
}