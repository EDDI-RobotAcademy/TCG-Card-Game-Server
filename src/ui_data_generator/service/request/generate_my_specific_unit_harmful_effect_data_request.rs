use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateMySpecificUnitHarmfulEffectDataRequest {
    my_unit_index: i32,
    my_unit_harmful_status_list: Vec<ExtraEffect>,
}

impl GenerateMySpecificUnitHarmfulEffectDataRequest {
    pub fn new(my_unit_index: i32,my_unit_harmful_status_list: Vec<ExtraEffect>) -> Self {
        GenerateMySpecificUnitHarmfulEffectDataRequest {
            my_unit_index,
            my_unit_harmful_status_list
        }
    }

    pub fn get_my_unit_index(&self) -> i32 { self.my_unit_index }

    pub fn get_harmful_status_list(&self) -> &Vec<ExtraEffect>{
        &self.my_unit_harmful_status_list
    }
}