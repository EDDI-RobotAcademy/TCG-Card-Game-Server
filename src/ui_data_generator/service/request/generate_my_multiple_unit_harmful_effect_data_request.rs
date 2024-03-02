use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateMyMultipleUnitHarmfulEffectDataRequest {
    my_unit_harmful_status_tuple_list: Vec<(i32, Vec<ExtraEffect>)>,
}

impl GenerateMyMultipleUnitHarmfulEffectDataRequest {
    pub fn new(my_unit_harmful_status_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) -> Self {
        GenerateMyMultipleUnitHarmfulEffectDataRequest {
            my_unit_harmful_status_tuple_list
        }
    }
    pub fn get_my_unit_harmful_status_tuple_list(&self) -> Vec<(i32, Vec<ExtraEffect>)> {
        self.my_unit_harmful_status_tuple_list.clone()
    }
}