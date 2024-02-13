use crate::game_field_unit::entity::extra_effect::ExtraEffect;

#[derive(Debug, Clone, PartialEq)]
pub struct HarmfulStatusEffect {
    effect: ExtraEffect,
    status_duration_turn: i32,
    effect_damage: i32,
    reuse_turn: i32,
}

impl HarmfulStatusEffect {
    pub fn new(effect: ExtraEffect, status_duration_turn: i32, effect_damage: i32, reuse_turn: i32) -> HarmfulStatusEffect {
        HarmfulStatusEffect {
            effect: effect.clone(),
            status_duration_turn,
            effect_damage,
            reuse_turn
        }
    }

    pub fn get_harmful_effect(&self) -> &ExtraEffect {
        &self.effect
    }

    pub fn set_status_duration_turn(&mut self, status_duration_turn: i32) {
        self.status_duration_turn = status_duration_turn;
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

    // TODO: 가만 보니까 어차피 duration 만료되어 효과 삭제 된 이후 의미가 없음
    pub fn set_reuse_turn(&mut self, reuse_turn: i32) {
        self.reuse_turn = reuse_turn;
    }
}
