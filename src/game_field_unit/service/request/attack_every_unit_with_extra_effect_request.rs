use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;

#[derive(Debug)]
pub struct AttackEveryUnitWithExtraEffectRequest {
    opponent_unique_id: i32,
    damage: i32,
    extra_status_effect_list: Vec<ExtraStatusEffect>,
}

impl AttackEveryUnitWithExtraEffectRequest {
    pub fn new(opponent_unique_id: i32,
               damage: i32,
               extra_status_effect_list: Vec<ExtraStatusEffect>) -> Self {
        AttackEveryUnitWithExtraEffectRequest {
            opponent_unique_id,
            damage,
            extra_status_effect_list,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn get_extra_status_effect_list(&self) -> &Vec<ExtraStatusEffect> {
        &self.extra_status_effect_list
    }
}