use crate::game_tomb::entity::tomb_card::TombCard;

#[derive(Debug)]
pub struct TombCardList {
    tomb_card_list: Vec<TombCard>,
}

impl TombCardList {
    pub fn new() -> TombCardList {
        TombCardList {
            tomb_card_list: Vec::new(),
        }
    }

    pub fn add_tomb_card(&mut self, card: TombCard) {
        self.tomb_card_list.push(card);
    }

    pub fn get_tomb_card_list(&self) -> &Vec<TombCard> {
        &self.tomb_card_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lost_zone_card_list() {
        let mut tomb_card_list = TombCardList::new();

        let card1 = TombCard::new(33);
        let card2 = TombCard::new(7);
        tomb_card_list.add_tomb_card(card1);
        tomb_card_list.add_tomb_card(card2);

        let cards = tomb_card_list.get_tomb_card_list();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].get_card(), 33);
        assert_eq!(cards[1].get_card(), 7);

        println!("{:?}", tomb_card_list);
    }
}