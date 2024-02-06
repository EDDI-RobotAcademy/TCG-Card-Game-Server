use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_hand::entity::game_hand::GameHand;
use crate::game_hand::entity::game_hand_card::GameHandCard;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;

pub struct GameHandRepositoryImpl {
    game_hand_map: IndexMap<i32, GameHand>,
}

impl GameHandRepositoryImpl {
    pub fn new() -> Self {
        GameHandRepositoryImpl {
            game_hand_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_hand_map(&mut self) -> &mut IndexMap<i32, GameHand> {
        &mut self.game_hand_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameHandRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameHandRepository for GameHandRepositoryImpl {
    fn create_game_hand_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameHandRepositoryImpl: create_game_hand_object()");

        let new_game_hand_map = GameHand::new();
        self.game_hand_map.insert(account_unique_id, new_game_hand_map);

        true
    }

    fn add_card_list_to_hand(&mut self, account_unique_id: i32, card_list: Vec<i32>) -> bool {
        println!("GameHandRepositoryImpl: add_card_list_to_hand()");

        if let Some(game_hand) = self.game_hand_map.get_mut(&account_unique_id) {
            game_hand.add_card_list_to_hand(card_list);
            return true
        }

        return false
    }

    fn remove_card_list_from_hand(&mut self, account_unique_id: i32, card_list: Vec<i32>) -> bool {
        if let Some(game_hand) = self.game_hand_map.get_mut(&account_unique_id) {
            game_hand.remove_card_list_from_hand(card_list);
            return true
        }

        return false
    }

    fn use_specific_card(&mut self, account_unique_id: i32, card_number: i32) -> Option<GameHandCard> {
        if let Some(game_hand) = self.game_hand_map.get_mut(&account_unique_id) {
            game_hand.get_specific_card(card_number)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_field_unit_object() {
        let mut game_hand_repository = GameHandRepositoryImpl::new();
        let result = game_hand_repository.create_game_hand_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_hand_repository.get_game_hand_map());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameHandRepositoryImpl::get_instance();

        let mut lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.get_game_hand_map().len(), 0);
    }

    #[tokio::test]
    async fn test_add_card_list_to_hand_in_repository() {
        let mut game_hand_repository = GameHandRepositoryImpl::new();
        game_hand_repository.create_game_hand_object(1);

        let card_list = vec![11, 22, 33];
        let result = game_hand_repository.add_card_list_to_hand(1, card_list.clone());

        assert!(result);

        let game_hand_map = game_hand_repository.get_game_hand_map();
        let game_hand = game_hand_map.get(&1).unwrap();
        let unit_list_in_game_hand = game_hand.get_all_card_list_in_game_hand();

        println!("GameHandRepositoryImpl - Content after adding cards: {:?}", unit_list_in_game_hand);

        assert_eq!(unit_list_in_game_hand.len(), card_list.len());
        assert_eq!(unit_list_in_game_hand[0].get_card(), card_list[0]);
        assert_eq!(unit_list_in_game_hand[1].get_card(), card_list[1]);
        assert_eq!(unit_list_in_game_hand[2].get_card(), card_list[2]);
    }

    #[tokio::test]
    async fn test_get_specific_card_in_repository() {
        let mut game_hand_repository = GameHandRepositoryImpl::new();
        game_hand_repository.create_game_hand_object(1);

        let card_list = vec![11, 22, 33];
        game_hand_repository.add_card_list_to_hand(1, card_list.clone());

        let specific_card = game_hand_repository.use_specific_card(1, 22);

        assert!(specific_card.is_some());
        assert_eq!(specific_card.unwrap().get_card(), 22);

        println!("{:?}", game_hand_repository.get_game_hand_map());
    }
}
