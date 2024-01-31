use crate::game_hand::entity::hand_card::HandCard;
use crate::game_hand::entity::hand_card_list::HandCardList;

#[derive(Debug)]
pub struct GameHand {
    game_hand: HandCardList,
}

impl GameHand {
    pub fn new() -> GameHand {
        GameHand { game_hand: HandCardList::new() }
    }

    pub fn add_card_to_game_hand(&mut self, hand_card: HandCard) {
        self.game_hand.add_hand_card(hand_card);
    }

    pub fn get_all_card_list_in_game_hand(&self) -> &Vec<HandCard> {
        self.game_hand.get_all_hand_card_list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_deck() {
        let mut game_hand = GameHand::new();

        let hand_card1 = HandCard::new(56);
        let hand_card2 = HandCard::new(78);

        game_hand.add_card_to_game_hand(hand_card1);
        game_hand.add_card_to_game_hand(hand_card2);

        let unit_list_in_game_hand = game_hand.get_all_card_list_in_game_hand();

        assert_eq!(unit_list_in_game_hand.len(), 2);
        assert_eq!(unit_list_in_game_hand[0].get_card(), 56);
        assert_eq!(unit_list_in_game_hand[1].get_card(), 78);

        println!("{:?}", unit_list_in_game_hand);
    }
}
