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
}