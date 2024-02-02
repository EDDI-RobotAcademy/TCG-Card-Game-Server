use crate::game_hand::entity::hand_card::GameHandCard;
use crate::game_hand::entity::hand_card_list::GameHandCardList;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_deck() {
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

        // Create a vector of integers to represent card values
        let card_values = vec![11, 22, 33, 44, 55];

        // Add the card values to the game hand
        game_hand.add_card_list_to_hand(card_values.clone());

        // Retrieve the card list from the game hand
        let unit_list_in_game_hand = game_hand.get_all_card_list_in_game_hand();

        // Assert that the length of the card list is as expected
        assert_eq!(unit_list_in_game_hand.len(), card_values.len());

        // Assert that each card in the hand has the correct value
        for (index, &value) in card_values.iter().enumerate() {
            assert_eq!(unit_list_in_game_hand[index].get_card(), value);
        }

        // Print the content of the game hand for manual inspection
        println!("{:?}", unit_list_in_game_hand);
    }
}
