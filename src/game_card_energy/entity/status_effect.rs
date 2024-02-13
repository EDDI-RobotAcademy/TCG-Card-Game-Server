use crate::game_card_energy::entity::effect::Effect;

#[derive(Debug, Clone, PartialEq)]
pub struct StatusEffect {
    effect: Effect,
    status_duration_turn: i32,
    effect_damage: i32,
    reuse_turn: i32,
}

impl StatusEffect {
    pub fn new(effect: Effect, status_duration_turn: i32, effect_damage: i32, reuse_turn: i32) -> StatusEffect {
        StatusEffect {
            effect: effect.clone(),
            status_duration_turn,
            effect_damage,
            reuse_turn
        }
    }

    pub fn get_effect(&self) -> &Effect {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_effect() {
        let effect = Effect::Darkfire;
        let status_duration_turn = 3;
        let effect_damage = 10;

        let status_effect = StatusEffect::new(effect.clone(), status_duration_turn, effect_damage, -1);

        assert_eq!(status_effect.get_effect(), &effect);
        assert_eq!(status_effect.get_status_duration_turn(), status_duration_turn);
        assert_eq!(status_effect.get_effect_damage(), effect_damage);

        println!("Effect: {:?}", status_effect.get_effect());
        println!("Status Duration Turn: {}", status_effect.get_status_duration_turn());
        println!("Effect Damage: {}", status_effect.get_effect_damage());
    }
}
