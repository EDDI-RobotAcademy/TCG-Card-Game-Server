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
}
