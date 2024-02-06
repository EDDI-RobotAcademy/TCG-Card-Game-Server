use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::energy_from_deck::EnergyFromDeck;

#[derive(PartialEq)]
pub struct GameCardSupportEffect {
    energy_from_deck: EnergyFromDeck,
}

impl GameCardSupportEffect {
    pub fn new(race: RaceEnum, energy_count: i32) -> Self {
        GameCardSupportEffect {
            energy_from_deck: EnergyFromDeck::new(race, energy_count)
        }
    }

    pub fn get_energy_from_deck(&self) -> &EnergyFromDeck {
        &self.energy_from_deck
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
