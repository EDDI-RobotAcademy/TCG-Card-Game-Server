use crate::game_hand::entity::game_hand_card::GameHandCard;
use crate::game_hand::entity::game_hand_card_list::GameHandCardList;

#[derive(Debug)]
pub struct GameHand {
    game_hand: GameHandCardList,
}

impl GameHand {
    pub fn new() -> GameHand {
        GameHand { game_hand: GameHandCardList::new() }
    }

    pub fn add_card_to_game_hand(&mut self, hand_card: GameHandCard) {
        self.game_hand.add_hand_card(hand_card);
    }

    pub fn add_card_list_to_hand(&mut self, card_list: Vec<i32>) {
        for card_value in card_list {
            let hand_card = GameHandCard::new(card_value);
            self.game_hand.add_hand_card(hand_card);
        }
    }

    pub fn get_all_card_list_in_game_hand(&self) -> &Vec<GameHandCard> {
        self.game_hand.get_all_hand_card_list()
    }

    pub fn get_specific_card(&mut self, card_number: i32) -> Option<GameHandCard> {
        self.game_hand.get_specific_hand_card(card_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_hand() {
        let mut game_hand = GameHand::new();

        let hand_card1 = GameHandCard::new(56);
        let hand_card2 = GameHandCard::new(78);

        game_hand.add_card_to_game_hand(hand_card1);
        game_hand.add_card_to_game_hand(hand_card2);

        let unit_list_in_game_hand = game_hand.get_all_card_list_in_game_hand();

        assert_eq!(unit_list_in_game_hand.len(), 2);
        assert_eq!(unit_list_in_game_hand[0].get_card(), 56);
        assert_eq!(unit_list_in_game_hand[1].get_card(), 78);

        println!("{:?}", unit_list_in_game_hand);
    }

    #[test]
    fn test_add_card_list_to_hand() {
        let mut game_hand = GameHand::new();
        let card_values = vec![11, 22, 33, 44, 55];

        game_hand.add_card_list_to_hand(card_values.clone());

        let unit_list_in_game_hand = game_hand.get_all_card_list_in_game_hand();
        assert_eq!(unit_list_in_game_hand.len(), card_values.len());

        for (index, &value) in card_values.iter().enumerate() {
            assert_eq!(unit_list_in_game_hand[index].get_card(), value);
        }

        println!("{:?}", unit_list_in_game_hand);
    }

    #[test]
    fn test_get_specific_card() {
        let mut game_hand = GameHand::new();

        let card_values = vec![11, 22, 33, 44, 55];
        game_hand.add_card_list_to_hand(card_values.clone());

        let specific_card = game_hand.get_specific_card(33);
        assert!(specific_card.is_some());
        assert_eq!(specific_card.unwrap().get_card(), 33);

        println!("{:?}", game_hand.get_all_card_list_in_game_hand());
    }
}
