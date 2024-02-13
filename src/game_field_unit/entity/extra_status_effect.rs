use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug, Clone, PartialEq)]
pub struct ExtraStatusEffect {
    effect: ExtraEffect,
    status_duration_turn: i32,
    effect_damage: i32,
    reuse_turn: i32,
}

impl ExtraStatusEffect {
    pub fn new(effect: ExtraEffect, status_duration_turn: i32, effect_damage: i32, reuse_turn: i32) -> ExtraStatusEffect {
        ExtraStatusEffect {
            effect: effect.clone(),
            status_duration_turn,
            effect_damage,
            reuse_turn
        }
    }

    pub fn get_extra_effect(&self) -> &ExtraEffect {
        &self.effect
    }

    pub fn get_status_duration_turn(&self) -> i32 {
        self.status_duration_turn
    }

    pub fn get_effect_damage(&self) -> i32 {
        self.effect_damage
    }

    pub fn get_reuse_turn(&self) -> i32 {
        self.reuse_turn
    }
}