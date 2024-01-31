use crate::game_hand::entity::hand_card::HandCard;

#[derive(Debug)]
pub struct HandCardList {
    hand_card_list: Vec<HandCard>,
}

impl HandCardList {
    pub fn new() -> HandCardList {
        HandCardList { hand_card_list: Vec::new() }
    }

    pub fn add_hand_card(&mut self, card: HandCard) {
        self.hand_card_list.push(card);
    }

    pub fn get_all_hand_card_list(&self) -> &Vec<HandCard> {
        &self.hand_card_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_card_list() {
        let mut hand_card_list = HandCardList::new();

        let hand_card1 = HandCard::new(3);
        let hand_card2 = HandCard::new(7);
        hand_card_list.add_hand_card(hand_card1);
        hand_card_list.add_hand_card(hand_card2);

        let hand_card_list = hand_card_list.get_all_hand_card_list();
        assert_eq!(hand_card_list.len(), 2);
        assert_eq!(hand_card_list[0].get_card(), 3);
        assert_eq!(hand_card_list[1].get_card(), 7);

        println!("{:?}", hand_card_list);
    }
}
