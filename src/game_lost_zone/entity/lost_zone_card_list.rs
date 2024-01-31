use crate::game_lost_zone::entity::lost_zone_card::LostZoneCard;

#[derive(Debug)]
pub struct LostZoneCardList {
    lost_zone_card_list: Vec<LostZoneCard>,
}

impl LostZoneCardList {
    pub fn new() -> LostZoneCardList {
        LostZoneCardList {
            lost_zone_card_list: Vec::new(),
        }
    }

    pub fn add_lost_zone_card(&mut self, card: LostZoneCard) {
        self.lost_zone_card_list.push(card);
    }

    pub fn get_lost_zone_cards(&self) -> &Vec<LostZoneCard> {
        &self.lost_zone_card_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lost_zone_card_list() {
        let mut lost_zone_card_list = LostZoneCardList::new();

        let card1 = LostZoneCard::new(42);
        let card2 = LostZoneCard::new(10);
        lost_zone_card_list.add_lost_zone_card(card1);
        lost_zone_card_list.add_lost_zone_card(card2);

        let cards = lost_zone_card_list.get_lost_zone_cards();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].get_card(), 42);
        assert_eq!(cards[1].get_card(), 10);

        println!("{:?}", lost_zone_card_list);
    }
}