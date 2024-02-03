use crate::game_hand::entity::game_hand_card::GameHandCard;

#[derive(Debug)]
pub struct GameHandCardList {
    hand_card_list: Vec<GameHandCard>,
}

impl GameHandCardList {
    pub fn new() -> GameHandCardList {
        GameHandCardList { hand_card_list: Vec::new() }
    }

    pub fn add_hand_card(&mut self, card: GameHandCard) {
        self.hand_card_list.push(card);
    }

    pub fn get_all_hand_card_list(&self) -> &Vec<GameHandCard> {
        &self.hand_card_list
    }

    pub fn get_specific_hand_card(&mut self, card_number: i32) -> Option<GameHandCard> {
        if let Some(index) = self.hand_card_list.iter().position(|card| card.get_card() == card_number) {
            Some(self.hand_card_list.remove(index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_hand::entity::game_hand_card_list;
    use super::*;

    #[test]
    fn test_hand_card_list() {
        let mut hand_card_list = GameHandCardList::new();

        let hand_card1 = GameHandCard::new(3);
        let hand_card2 = GameHandCard::new(7);
        hand_card_list.add_hand_card(hand_card1);
        hand_card_list.add_hand_card(hand_card2);

        let hand_card_list = hand_card_list.get_all_hand_card_list();
        assert_eq!(hand_card_list.len(), 2);
        assert_eq!(hand_card_list[0].get_card(), 3);
        assert_eq!(hand_card_list[1].get_card(), 7);

        println!("{:?}", hand_card_list);
    }

    #[test]
    fn test_get_single_hand_card() {
        let mut hand_card_list = GameHandCardList::new();

        let hand_card1 = GameHandCard::new(3);
        let hand_card2 = GameHandCard::new(7);
        hand_card_list.add_hand_card(hand_card1);
        hand_card_list.add_hand_card(hand_card2);

        let all_of_hand_card_list = hand_card_list.get_all_hand_card_list();
        assert_eq!(all_of_hand_card_list.len(), 2);
        assert_eq!(all_of_hand_card_list[0].get_card(), 3);
        assert_eq!(all_of_hand_card_list[1].get_card(), 7);

        println!("{:?}", all_of_hand_card_list);

        let specific_card = hand_card_list.get_specific_hand_card(3);
        assert_eq!(specific_card.unwrap().get_card(), 3);

        let all_of_hand_card_list = hand_card_list.get_all_hand_card_list();
        println!("{:?}", all_of_hand_card_list);
    }
}
