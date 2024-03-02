use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug)]
pub struct GenerateOpponentSpecificUnitExtraEffectDataRequest {
    opponent_unit_index: i32,
    opponent_unit_extra_effect_list: Vec<ExtraEffect>,
}

impl GenerateOpponentSpecificUnitExtraEffectDataRequest {
    pub fn new(opponent_unit_index: i32,opponent_unit_extra_effect_list: Vec<ExtraEffect>) -> Self {
        GenerateOpponentSpecificUnitExtraEffectDataRequest {
            opponent_unit_index,
            opponent_unit_extra_effect_list
        }
    }

    pub fn get_opponent_unit_index(&self) -> i32 { self.opponent_unit_index }

    pub fn get_opponent_unit_extra_effect_list(&self) -> &Vec<ExtraEffect>{
        &self.opponent_unit_extra_effect_list
    }
}