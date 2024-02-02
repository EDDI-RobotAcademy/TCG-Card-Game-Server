use std::time::SystemTime;
use rand::prelude::StdRng;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::game_deck::entity::game_deck_card::GameDeckCard;

#[derive(Debug)]
pub struct GameDeckCardList {
    card_list: Vec<GameDeckCard>,
}

impl GameDeckCardList {
    pub fn new() -> GameDeckCardList {
        GameDeckCardList { card_list: Vec::new() }
    }

    pub fn add_card(&mut self, card: GameDeckCard) {
        self.card_list.push(card);
    }

    pub fn get_all_card_list(&self) -> &Vec<GameDeckCard> {
        &self.card_list
    }

    pub fn set_card_list(&mut self, card_list: Vec<GameDeckCard>) {
        self.card_list = card_list;
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.card_list.shuffle(&mut rng);
    }

    // pub fn shuffle(&mut self) {
    //     let seed = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .expect("Time went backwards")
    //         .as_nanos() as u64;
    //
    //     let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    //     self.card_list.shuffle(&mut rng);
    // }

    pub fn draw_deck_card_list(&mut self, draw_count: usize) -> Vec<i32> {
        let drawn_card_list: Vec<i32> = self.card_list.iter().take(draw_count).map(|card| card.get_card()).collect();

        let mut updated_card_list = self.card_list.clone();
        updated_card_list.drain(0..draw_count);
        self.card_list = updated_card_list;

        drawn_card_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_card_list() {
        let mut deck_card_list = GameDeckCardList::new();

        let card1 = GameDeckCard::new(42);
        let card2 = GameDeckCard::new(10);
        deck_card_list.add_card(card1);
        deck_card_list.add_card(card2);

        let card_list = deck_card_list.get_all_card_list();
        assert_eq!(card_list.len(), 2);
        assert_eq!(card_list[0].get_card(), 42);
        assert_eq!(card_list[1].get_card(), 10);

        println!("{:?}", deck_card_list);
    }

    #[test]
    fn test_draw_deck_card_list() {
        let mut deck_card_list = GameDeckCardList::new();

        let card1 = GameDeckCard::new(42);
        let card2 = GameDeckCard::new(10);
        let card3 = GameDeckCard::new(7);

        deck_card_list.add_card(card1);
        deck_card_list.add_card(card2);
        deck_card_list.add_card(card3);

        let initial_card_list = deck_card_list.get_all_card_list().clone();
        println!("Initial card list: {:?}", initial_card_list);

        let drawn_cards = deck_card_list.draw_deck_card_list(2);
        println!("Drawn cards: {:?}", drawn_cards);

        let remaining_card_list = deck_card_list.get_all_card_list().clone();
        println!("Remaining card list: {:?}", remaining_card_list);

        assert_eq!(drawn_cards.len(), 2);
        assert_eq!(initial_card_list.len() - drawn_cards.len(), remaining_card_list.len());
    }
}