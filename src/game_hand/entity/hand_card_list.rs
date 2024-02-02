use crate::game_hand::entity::hand_card::GameHandCard;

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
}

#[cfg(test)]
mod tests {
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
}
