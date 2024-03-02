use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;

#[derive(Debug, Clone)]
pub struct AcquireUnitHarmfulStatusEffectResponse {
    harmful_status_effect_list: Vec<HarmfulStatusEffect>,
}

impl AcquireUnitHarmfulStatusEffectResponse {
    pub fn new(harmful_status_effect_list: Vec<HarmfulStatusEffect>) -> Self {
        Self { harmful_status_effect_list }
    }

    pub fn get_harmful_status_effect_list(&self) -> &Vec<HarmfulStatusEffect> {
        &self.harmful_status_effect_list
    }

    pub fn get_harmful_effect_list(&self) -> Vec<ExtraEffect> {
        let mut harmful_effect_list = Vec::new();

        for harmful_status_effect in &self.harmful_status_effect_list {
            harmful_effect_list.push(harmful_status_effect.get_harmful_effect().clone());
        }

        harmful_effect_list
    }
}