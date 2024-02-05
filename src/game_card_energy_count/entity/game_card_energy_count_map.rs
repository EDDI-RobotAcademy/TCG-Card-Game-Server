use indexmap::IndexMap;

use crate::game_card_energy_count::entity::game_card_energy_count::GameCardEnergyCount;

#[derive(Debug)]
pub struct GameCardEnergyCountMap {
    energy_count_map: IndexMap<i32, GameCardEnergyCount>
}

impl GameCardEnergyCountMap {
    pub fn new() -> Self { GameCardEnergyCountMap { energy_count_map: IndexMap::new() } }
    pub fn create_game_card_energy_count_object(&mut self, card: i32) {
        let card_energy_count = GameCardEnergyCount::new();
        self.energy_count_map.insert(card, card_energy_count);
    }
    pub fn set_game_card_energy_count_from_data(&mut self, deck: Vec<i32>) {
        for card in deck {
            self.energy_count_map.entry(card).or_insert(GameCardEnergyCount::new());
        }
    }
    pub fn add_energy_to_game_card(&mut self, card: i32) {
        if let Some(game_card_energy_count) = self.energy_count_map.get_mut(&card) {
            game_card_energy_count.add_energy_to_card()
        }
    }
    pub fn get_game_card_energy_count(&mut self) -> &mut IndexMap<i32, GameCardEnergyCount> {
        &mut self.energy_count_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::game_card_energy_count::entity::game_card_energy_count_map::GameCardEnergyCountMap;

    #[test]
    async fn test_game_card_energy_count() {
        let mut game_card_energy_count_map = GameCardEnergyCountMap::new();

        game_card_energy_count_map.create_game_card_energy_count_object(3);
        game_card_energy_count_map.create_game_card_energy_count_object(8);

        println!("{:?}", game_card_energy_count_map.get_game_card_energy_count());

        for _ in 0..3 {
            game_card_energy_count_map.add_energy_to_game_card(3);
        }

        println!("{:?}", game_card_energy_count_map.get_game_card_energy_count());
    }
}