use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_deck::entity::game_deck::GameDeck;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;

pub struct GameDeckRepositoryImpl {
    game_deck_map: IndexMap<i32, GameDeck>,
}

impl GameDeckRepositoryImpl {
    pub fn new() -> Self {
        GameDeckRepositoryImpl {
            game_deck_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_deck_map(&mut self) -> &mut IndexMap<i32, GameDeck> {
        &mut self.game_deck_map
    }

    pub fn set_game_deck_from_data(&mut self, account_unique_id: i32, data: Vec<i32>) {
        let mut game_deck_map = self.get_game_deck_map();

        let game_deck = game_deck_map.entry(account_unique_id).or_insert(GameDeck::new());
        game_deck.set_card_list_from_data(data);
    }

    pub fn get_game_deck_card_ids(&self, account_id: i32) -> Vec<i32> {
        if let Some(game_deck) = self.game_deck_map.get(&account_id) {
            game_deck.get_card_ids()
        } else {
            Vec::new()
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameDeckRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameDeckRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameDeckRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameDeckRepository for GameDeckRepositoryImpl {
    fn create_game_deck_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameDeckRepositoryImpl: create_game_deck_object()");

        let new_game_deck_map = GameDeck::new();
        self.game_deck_map.insert(account_unique_id, new_game_deck_map);

        true
    }

    fn shuffle_game_deck(&mut self, account_unique_id: i32) -> bool {
        if let Some(game_deck) = self.game_deck_map.get_mut(&account_unique_id) {
            game_deck.shuffle_game_deck();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_game_deck_repository_impl_create_game_deck_object() {
        let mut repo = GameDeckRepositoryImpl::new();
        let account_unique_id = 1;
        assert!(repo.create_game_deck_object(account_unique_id));
        let game_deck_map = repo.get_game_deck_map();
        assert!(game_deck_map.contains_key(&account_unique_id));
    }
}