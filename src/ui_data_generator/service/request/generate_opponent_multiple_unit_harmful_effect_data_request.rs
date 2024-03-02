use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateOpponentMultipleUnitHarmfulEffectDataRequest {
    opponent_unit_harmful_status_tuple_list: Vec<(i32, Vec<ExtraEffect>)>,
}

impl GenerateOpponentMultipleUnitHarmfulEffectDataRequest {
    pub fn new(opponent_unit_harmful_status_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) -> Self {
        GenerateOpponentMultipleUnitHarmfulEffectDataRequest {
            opponent_unit_harmful_status_tuple_list
        }
    }
    pub fn get_opponent_unit_harmful_status_tuple_list(&self) -> Vec<(i32, Vec<ExtraEffect>)> {
        self.opponent_unit_harmful_status_tuple_list.clone()
    }
}