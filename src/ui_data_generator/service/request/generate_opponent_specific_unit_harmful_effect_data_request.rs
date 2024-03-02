use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateOpponentSpecificUnitHarmfulEffectDataRequest {
    opponent_unit_index: i32,
    opponent_unit_harmful_status_list: Vec<ExtraEffect>,
}

impl GenerateOpponentSpecificUnitHarmfulEffectDataRequest {
    pub fn new(opponent_unit_index: i32,opponent_unit_harmful_status_list: Vec<ExtraEffect>) -> Self {
        GenerateOpponentSpecificUnitHarmfulEffectDataRequest {
            opponent_unit_index,
            opponent_unit_harmful_status_list
        }
    }

    pub fn get_opponent_unit_index(&self) -> i32 { self.opponent_unit_index }

    pub fn get_opponent_unit_harmful_status_list(&self) -> &Vec<ExtraEffect>{
        &self.opponent_unit_harmful_status_list
    }
}