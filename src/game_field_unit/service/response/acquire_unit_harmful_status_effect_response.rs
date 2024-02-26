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
}