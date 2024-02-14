use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;

#[derive(Debug)]
pub struct AttackTargetUnitWithExtraEffectRequest {
    opponent_unique_id: i32,
    damage: i32,
    extra_status_effect_list: Vec<ExtraStatusEffect>,
    target_unit_index: i32
}

impl AttackTargetUnitWithExtraEffectRequest {
    pub fn new(opponent_unique_id: i32,
               damage: i32,
               extra_status_effect_list: Vec<ExtraStatusEffect>,
               target_unit_index: i32) -> Self {

        AttackTargetUnitWithExtraEffectRequest {
            opponent_unique_id,
            damage,
            extra_status_effect_list,
            target_unit_index
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

    pub fn get_target_unit_index(&self) -> i32 {
        self.target_unit_index
    }
}
