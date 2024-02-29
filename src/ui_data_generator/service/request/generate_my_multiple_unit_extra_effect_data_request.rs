use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateMyMultipleUnitExtraEffectDataRequest {
    my_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>,
}

impl GenerateMyMultipleUnitExtraEffectDataRequest {
    pub fn new(my_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) -> Self {
        GenerateMyMultipleUnitExtraEffectDataRequest {
            my_unit_extra_effect_tuple_list
        }
    }
    pub fn get_my_unit_extra_effect_tuple_list(&self) -> Vec<(i32, Vec<ExtraEffect>)> {
        self.my_unit_extra_effect_tuple_list.clone()
    }
}