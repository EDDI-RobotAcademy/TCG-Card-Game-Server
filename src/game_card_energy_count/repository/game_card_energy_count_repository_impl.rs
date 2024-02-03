use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::game_card_energy_count::entity::game_card_energy_count_map::GameCardEnergyCountMap;
use crate::game_card_energy_count::repository::game_card_energy_count_repository::GameCardEnergyCountRepository;

pub struct GameCardEnergyCountRepositoryImpl {
    player_game_card_energy_count_map: IndexMap<i32, GameCardEnergyCountMap>
}

impl GameCardEnergyCountRepositoryImpl {
    pub fn new() -> Self {
        GameCardEnergyCountRepositoryImpl {
            player_game_card_energy_count_map: IndexMap::new(),
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardEnergyCountRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardEnergyCountRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardEnergyCountRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
    pub fn set_game_card_energy_count_from_data(&mut self, account_unique_id: i32, deck: Vec<i32>) {
        let mut player_game_card_energy_count_map =
            self.get_player_game_card_energy_count_map();

        let game_card_energy_count_map =
            player_game_card_energy_count_map.entry(account_unique_id).or_insert(GameCardEnergyCountMap::new());

        game_card_energy_count_map.set_game_card_energy_count_from_data(deck)
    }
    pub fn get_player_game_card_energy_count_map(&mut self) -> &mut IndexMap<i32, GameCardEnergyCountMap> {
        &mut self.player_game_card_energy_count_map
    }
}

impl GameCardEnergyCountRepository for GameCardEnergyCountRepositoryImpl {
    fn create_player_game_card_energy_count_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameCardEnergyCountRepositoryImpl: create_player_game_card_energy_count_object()");

        let new_game_card_energy_count_map = GameCardEnergyCountMap::new();
        self.player_game_card_energy_count_map.insert(account_unique_id, new_game_card_energy_count_map);

        true
    }
    fn add_energy_to_player_game_card(&mut self, account_unique_id: i32, card: i32) -> bool {
        println!("GameCardEnergyCountRepositoryImpl: add_energy_to_player_game_card()");

        if let Some(game_card_energy_count_map) =
            self.player_game_card_energy_count_map.get_mut(&account_unique_id) {
            game_card_energy_count_map.add_energy_to_game_card(card);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_game_card_energy_count() {
        let mut game_card_energy_count_repository = GameCardEnergyCountRepositoryImpl::new();

        let player1_game_card_energy_count_object =
            game_card_energy_count_repository.create_player_game_card_energy_count_object(1);

        let sample_deck1 = [19, 22, 36, 43, 51].to_vec();

        game_card_energy_count_repository.set_game_card_energy_count_from_data(1, sample_deck1);

        let result = game_card_energy_count_repository.add_energy_to_player_game_card(1, 43);

        assert!(player1_game_card_energy_count_object);
        assert!(result);

        println!("{:?}", game_card_energy_count_repository.get_player_game_card_energy_count_map());
    }
}