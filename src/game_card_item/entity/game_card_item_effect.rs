use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::required_energy::RequiredEnergy;
use crate::game_card_support::entity::energy_from_deck::EnergyFromDeck;

#[derive(PartialEq, Debug)]
pub struct GameCardItemEffect {
    required_energy: RequiredEnergy,
    alternatives_damage: i32,
    apply_target_minimum_grade: GradeEnum
}

impl GameCardItemEffect {
    pub fn new(required_energy_race: RaceEnum,
               required_energy_count: i32,
               alternatives_damage: i32,
               apply_target_minimum_grade: GradeEnum) -> Self {

        GameCardItemEffect {
            required_energy: RequiredEnergy::new(required_energy_race, required_energy_count),
            alternatives_damage,
            apply_target_minimum_grade
        }
    }

    pub fn set_alternatives_damage(&mut self, alternatives_damage: i32) {
        self.alternatives_damage = alternatives_damage;
    }

    pub fn get_required_energy(&self) -> &RequiredEnergy {
        &self.required_energy
    }

    pub fn get_alternatives_damage(&self) -> i32 {
        self.alternatives_damage
    }

    pub fn get_apply_target_minimum_grade(&self) -> GradeEnum {
        self.apply_target_minimum_grade
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_card_support_effect() {
        let game_card_item_effect = GameCardItemEffect::new(
            RaceEnum::Human, 5, 30, GradeEnum::Legend);
        println!("Energy From Deck: {:?}", game_card_item_effect);

        assert_eq!(game_card_item_effect.get_alternatives_damage(), 30);
        assert_eq!(game_card_item_effect.get_required_energy().get_required_energy_race(), &RaceEnum::Human);
        assert_eq!(game_card_item_effect.get_required_energy().get_required_energy_count(), 5);
        assert_eq!(game_card_item_effect.get_apply_target_minimum_grade(), GradeEnum::Legend);
    }
}
