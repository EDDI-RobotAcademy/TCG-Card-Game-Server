use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug, Clone, PartialEq)]
pub struct HarmfulStatusEffect {
    effect: ExtraEffect,
    status_duration_round: i32,
    effect_damage: i32,
    reapply_round: i32,
}

impl HarmfulStatusEffect {
    pub fn new(effect: ExtraEffect, status_duration_round: i32, effect_damage: i32, reapply_round: i32) -> HarmfulStatusEffect {
        HarmfulStatusEffect {
            effect: effect.clone(),
            status_duration_round,
            effect_damage,
            reapply_round
        }
    }

    pub fn get_harmful_effect(&self) -> &ExtraEffect {
        &self.effect
    }

    pub fn set_status_duration_round(&mut self, status_duration_round: i32) {
        self.status_duration_round = status_duration_round;
    }

    pub fn get_status_duration_round(&self) -> i32 {
        self.status_duration_round
    }

    pub fn get_effect_damage(&self) -> i32 {
        self.effect_damage
    }

    pub fn get_reapply_round(&self) -> i32 {
        self.reapply_round
    }

    pub fn set_reapply_round(&mut self, reapply_round: i32) {
        self.reapply_round = reapply_round
    }
}
