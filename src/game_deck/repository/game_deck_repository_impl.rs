use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_deck::entity::game_deck::GameDeck;
use crate::game_deck::entity::game_deck_card::GameDeckCard;
use crate::game_deck::entity::game_deck_card_list;
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

    pub fn get_game_deck_map(&mut self) -> &mut IndexMap<i32, GameDeck> {
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
        println!("GameDeckRepositoryImpl: shuffle_game_deck()");

        if let Some(game_deck) = self.game_deck_map.get_mut(&account_unique_id) {
            game_deck.shuffle_game_deck();
            true
        } else {
            false
        }
    }

    fn draw_deck_card(&mut self, account_unique_id: i32, draw_count: i32) -> Vec<i32> {
        println!("GameDeckRepositoryImpl: draw_deck_card()");

        if let Some(game_deck) = self.game_deck_map.get_mut(&account_unique_id) {
            return game_deck.draw_game_deck(draw_count as usize)
        }

        return Vec::new()
    }

    fn add_cards_to_deck(&mut self, account_unique_id: i32, cards: Vec<i32>) -> bool {
        println!("GameDeckRepositoryImpl: add_cards_to_deck()");

        if let Some(game_deck) = self.game_deck_map.get_mut(&account_unique_id) {
            for card in cards {
                let game_deck_card_form = GameDeckCard::new(card);
                game_deck.add_card_to_game_deck(game_deck_card_form)
            }
        }
        true
    }

    fn find_by_card_id_with_count(&mut self, account_id: i32, need_to_find_card_id: i32, card_count: i32) -> Vec<i32> {
        println!("GameDeckRepositoryImpl: find_by_card_id()");

        if let Some(game_deck) = self.game_deck_map.get_mut(&account_id) {
            return game_deck.find_cards_by_id_with_count(need_to_find_card_id, card_count as usize);
        }

        Vec::new()
    }


    fn get_remain_deck_card_count(&self, account_id: i32) -> i32 {
        if let Some(game_deck) = self.game_deck_map.get(&account_id)
        {
            game_deck.get_all_cards_in_game_deck().len() as i32
        }
        else
        {
            0i32
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

    #[tokio::test]
    async fn test_game_deck_repository_impl_draw_card() {
        let mut repo = GameDeckRepositoryImpl::new();
        let account_unique_id = 1;

        assert!(repo.create_game_deck_object(account_unique_id));

        let initial_cards = vec![1, 2, 3, 4, 5];
        repo.set_game_deck_from_data(account_unique_id, initial_cards.clone());

        let drawn_cards = repo.draw_deck_card(account_unique_id, 3);

        assert_eq!(drawn_cards, initial_cards.iter().take(3).cloned().collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn test_something_like_real_draw_deck() {
        let data = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
            30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
        ];

        let account_unique_id = 1;
        let repo = GameDeckRepositoryImpl::get_instance();
        let mut repo_guard = repo.lock().await;
        repo_guard.set_game_deck_from_data(account_unique_id, data.clone());

        assert!(repo_guard.shuffle_game_deck(account_unique_id));

        let current_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Shuffled cards: {:?}", current_cards);

        let drawn_cards = repo_guard.draw_deck_card(account_unique_id, 5);
        assert_eq!(drawn_cards.len(), 5);
        println!("Drawn cards: {:?}", drawn_cards);

        let remaining_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Remaining cards: {:?}", remaining_cards);
    }

    #[tokio::test]
    async fn test_find_card_from_deck() {
        let data = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
            30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
        ];

        let account_unique_id = 1;
        let repo = GameDeckRepositoryImpl::get_instance();
        let mut repo_guard = repo.lock().await;
        repo_guard.set_game_deck_from_data(account_unique_id, data.clone());

        assert!(repo_guard.shuffle_game_deck(account_unique_id));

        let current_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Shuffled cards: {:?}", current_cards);

        let drawn_cards = repo_guard.draw_deck_card(account_unique_id, 5);
        assert_eq!(drawn_cards.len(), 5);
        println!("Drawn cards: {:?}", drawn_cards);

        let remaining_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Remaining cards: {:?}", remaining_cards);

        let found_cards = repo_guard.find_by_card_id_with_count(account_unique_id, 93, 2);
        println!("Found cards with ID 3: {:?}", found_cards);

        let remaining_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Remaining cards: {:?}", remaining_cards);

        repo_guard.shuffle_game_deck(account_unique_id);
        let current_cards = repo_guard.get_game_deck_card_ids(account_unique_id);
        println!("Shuffled cards: {:?}", current_cards);
    }
}