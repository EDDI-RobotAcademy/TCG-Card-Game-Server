use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateMySpecificUnitExtraEffectDataRequest {
    my_unit_index: i32,
    my_unit_extra_effect_list: Vec<ExtraEffect>,
}

impl GenerateMySpecificUnitExtraEffectDataRequest {
    pub fn new(my_unit_index: i32,my_unit_extra_effect_list: Vec<ExtraEffect>) -> Self {
        GenerateMySpecificUnitExtraEffectDataRequest {
            my_unit_index,
            my_unit_extra_effect_list
        }
    }

    pub fn get_my_unit_index(&self) -> i32 { self.my_unit_index }

    pub fn get_extra_effect_list(&self) -> &Vec<ExtraEffect>{
        &self.my_unit_extra_effect_list
    }
}