use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::game_deck::entity::game_deck_card::GameDeckCard;
use crate::game_deck::entity::game_deck_card_list::GameDeckCardList;

#[derive(Debug)]
pub struct GameDeck {
    game_deck: GameDeckCardList,
}

impl GameDeck {
    pub fn new() -> GameDeck {
        GameDeck { game_deck: GameDeckCardList::new() }
    }

    pub fn add_card_to_game_deck(&mut self, card: GameDeckCard) {
        self.game_deck.add_card(card);
    }

    pub fn get_all_cards_in_game_deck(&self) -> &Vec<GameDeckCard> {
        self.game_deck.get_all_card_list()
    }

    pub fn shuffle_game_deck(&mut self) {
        self.game_deck.shuffle();
    }

    pub fn set_card_list_from_data(&mut self, data: Vec<i32>) {
        let game_deck_cards: Vec<GameDeckCard> = data
            .into_iter()
            .map(GameDeckCard::new)
            .collect();

        self.game_deck.set_card_list(game_deck_cards);
    }

    pub fn get_card_ids(&self) -> Vec<i32> {
        self.game_deck.get_all_card_list().iter()
            .map(|card| card.get_card())
            .collect()
    }

    pub fn draw_game_deck(&mut self, draw_count: usize) -> Vec<i32> {
        self.game_deck.draw_deck_card_list(draw_count)
    }

    pub fn find_cards_by_id_with_count(&mut self, wanna_find_id: i32, count: usize) -> Vec<i32> {
        self.game_deck.find_by_card_id_with_count(wanna_find_id, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_deck() {
        let mut game_deck = GameDeck::new();

        let card1 = GameDeckCard::new(33);
        let card2 = GameDeckCard::new(77);

        game_deck.add_card_to_game_deck(card1);
        game_deck.add_card_to_game_deck(card2);

        let cards_in_game_deck = game_deck.get_all_cards_in_game_deck();

        assert_eq!(cards_in_game_deck.len(), 2);
        assert_eq!(cards_in_game_deck[0].get_card(), 33);
        assert_eq!(cards_in_game_deck[1].get_card(), 77);

        println!("{:?}", game_deck);
    }

    #[test]
    fn test_game_deck_shuffle() {
        let mut game_deck = GameDeck::new();

        let card1 = GameDeckCard::new(33);
        let card2 = GameDeckCard::new(77);

        game_deck.add_card_to_game_deck(card1);
        game_deck.add_card_to_game_deck(card2);

        let cards_in_game_deck_before = game_deck.get_all_cards_in_game_deck().clone();
        println!("Before shuffle: {:?}", cards_in_game_deck_before);

        game_deck.shuffle_game_deck();

        let cards_in_game_deck_after = game_deck.get_all_cards_in_game_deck().clone();
        println!("After shuffle: {:?}", cards_in_game_deck_after);

        println!("{:?}", game_deck);
    }

    #[test]
    fn test_set_card_list_from_data() {
        let mut game_deck_object = GameDeck::new();

        let data = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
            30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
        ];

        game_deck_object.set_card_list_from_data(data);

        let cards_in_game_deck_after = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After setting cards from data: {:?}", cards_in_game_deck_after);

        game_deck_object.shuffle_game_deck();

        let cards_in_game_deck_after_shuffle = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After shuffle: {:?}", cards_in_game_deck_after_shuffle);

        println!("{:?}", game_deck_object);
    }

    #[test]
    fn test_draw_game_deck() {
        let mut game_deck_object = GameDeck::new();

        let data = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
            30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
        ];

        game_deck_object.set_card_list_from_data(data);

        let cards_in_game_deck_after = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After setting cards from data: {:?}", cards_in_game_deck_after);

        game_deck_object.shuffle_game_deck();
        let cards_in_game_deck_after_shuffle = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After shuffle: {:?}", cards_in_game_deck_after_shuffle);

        let drawn_cards = game_deck_object.draw_game_deck(5);
        println!("Drawn cards: {:?}", drawn_cards);

        let remaining_cards = game_deck_object.get_all_cards_in_game_deck();
        println!("Remaining cards: {:?}", remaining_cards);
    }

    #[test]
    fn test_find_cards_by_id_with_count() {
        let mut game_deck_object = GameDeck::new();

        let data = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
            30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
        ];

        game_deck_object.set_card_list_from_data(data);

        let cards_in_game_deck_after = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After setting cards from data: {:?}", cards_in_game_deck_after);

        game_deck_object.shuffle_game_deck();
        let cards_in_game_deck_after_shuffle = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After shuffle: {:?}", cards_in_game_deck_after_shuffle);

        let found_cards = game_deck_object.find_cards_by_id_with_count(93, 2);
        println!("Found cards with ID 8: {:?}", found_cards);

        let remaining_cards = game_deck_object.get_all_cards_in_game_deck();
        println!("Remaining cards: {:?}", remaining_cards);

        game_deck_object.shuffle_game_deck();
        let cards_in_game_deck_after_shuffle = game_deck_object.get_all_cards_in_game_deck().clone();
        println!("After shuffle: {:?}", cards_in_game_deck_after_shuffle);
    }
}
