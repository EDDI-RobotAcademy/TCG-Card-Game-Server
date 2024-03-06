use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;

#[derive(Debug, Clone)]
pub struct AcquireHarmfulStatusEffectOfAllUnitResponse {
    harmful_status_effect_list_of_all_unit: Vec<(i32, Vec<HarmfulStatusEffect>)>,
}

impl AcquireHarmfulStatusEffectOfAllUnitResponse {
    pub fn new(harmful_status_effect_list_of_all_unit: Vec<(i32, Vec<HarmfulStatusEffect>)>,) -> Self {
        Self { harmful_status_effect_list_of_all_unit }
    }

    pub fn get_harmful_status_effect_list_of_all_unit(&self) -> &Vec<(i32, Vec<HarmfulStatusEffect>)> {
        &self.harmful_status_effect_list_of_all_unit
    }

    pub fn get_harmful_effect_list_of_all_unit(&self) -> Vec<(i32, Vec<ExtraEffect>)> {
        let mut harmful_effect_list_with_index = Vec::new();

        if !self.harmful_status_effect_list_of_all_unit.is_empty() {
            for (unit_index, harmful_status_effect_list) in &self.harmful_status_effect_list_of_all_unit {
                let mut harmful_effect_list_of_unit = Vec::new();
                for harmful_status_effect in harmful_status_effect_list {
                    let harmful_effect = harmful_status_effect.get_harmful_effect().clone();
                    harmful_effect_list_of_unit.push(harmful_effect)
                }
                harmful_effect_list_with_index.push((*unit_index, harmful_effect_list_of_unit))
            }
        }

        harmful_effect_list_with_index
    }
}