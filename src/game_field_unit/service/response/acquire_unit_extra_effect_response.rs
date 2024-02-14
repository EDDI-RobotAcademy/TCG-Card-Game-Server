use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;

#[derive(Debug, Clone)]
pub struct AcquireUnitExtraEffectResponse {
    extra_status_effect_list: Vec<ExtraStatusEffect>,
}

impl AcquireUnitExtraEffectResponse {
    pub fn new(extra_status_effect_list: Vec<ExtraStatusEffect>) -> Self {
        Self { extra_status_effect_list }
    }

    pub fn get_extra_status_effect_list(&self) -> &Vec<ExtraStatusEffect> {
        &self.extra_status_effect_list
    }
}
