use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::energy_from_deck::EnergyFromDeck;

#[derive(PartialEq)]
pub struct GameCardSupportEffect {
    energy_from_deck: EnergyFromDeck,
    need_to_find_card_id: i32,
    need_to_draw_card_count: i32,
}

impl GameCardSupportEffect {
    pub fn new(race: RaceEnum, energy_count: i32) -> Self {
        GameCardSupportEffect {
            energy_from_deck: EnergyFromDeck::new(race, energy_count),
            need_to_find_card_id: -1,
            need_to_draw_card_count: -1,
        }
    }

    // Energy Boosting
    pub fn get_energy_from_deck(&self) -> &EnergyFromDeck {
        &self.energy_from_deck
    }
    pub fn get_need_to_find_card_id(&self) -> i32 {
        self.need_to_find_card_id
    }
    pub fn set_need_to_find_card_id(&mut self, need_to_find_card_id: i32) {
        self.need_to_find_card_id = need_to_find_card_id;
    }

    // Normal Draw
    pub fn get_need_to_draw_card_count(&self) -> i32 { self.need_to_draw_card_count }
    pub fn set_need_to_draw_card_count(&mut self, need_to_draw_card_count: i32) {
        self.need_to_draw_card_count = need_to_draw_card_count;
    }
}

// cfg 테스트를 위한 코드
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_card_support_effect() {
        let game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Human, 5);

        let energy_from_deck = game_card_support_effect.get_energy_from_deck();
        println!("Energy From Deck: {:?}", energy_from_deck);

        assert_eq!(energy_from_deck.get_race(), &RaceEnum::Human);
        assert_eq!(energy_from_deck.get_energy_count(), 5);
    }
}
