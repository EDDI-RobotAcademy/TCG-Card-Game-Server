use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateOpponentMultipleUnitExtraEffectDataRequest {
    opponent_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>,
}

impl GenerateOpponentMultipleUnitExtraEffectDataRequest {
    pub fn new(opponent_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) -> Self {
        GenerateOpponentMultipleUnitExtraEffectDataRequest {
            opponent_unit_extra_effect_tuple_list
        }
    }
    pub fn get_opponent_unit_extra_effect_tuple_list(&self) -> Vec<(i32, Vec<ExtraEffect>)> {
        self.opponent_unit_extra_effect_tuple_list.clone()
    }
}