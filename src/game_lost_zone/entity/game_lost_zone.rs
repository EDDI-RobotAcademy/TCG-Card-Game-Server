use crate::game_lost_zone::entity::lost_zone_card::LostZoneCard;
use crate::game_lost_zone::entity::lost_zone_card_list::LostZoneCardList;

#[derive(Debug)]
pub struct GameLostZone {
    game_lost_zone: LostZoneCardList,
}

impl GameLostZone {
    pub fn new() -> GameLostZone {
        GameLostZone {
            game_lost_zone: LostZoneCardList::new(),
        }
    }

    pub fn add_lost_zone_card(&mut self, card_id: i32) {
        let lost_zone_card = LostZoneCard::new(card_id);
        self.game_lost_zone.add_lost_zone_card(lost_zone_card);
    }

    pub fn get_lost_zone_card_list(&self) -> &LostZoneCardList {
        &self.game_lost_zone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_lost_zone() {
        let mut game_lost_zone = GameLostZone::new();

        game_lost_zone.add_lost_zone_card(42);
        game_lost_zone.add_lost_zone_card(10);

        let card_list = game_lost_zone.get_lost_zone_card_list();
        let cards = card_list.get_lost_zone_card_list();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].get_card(), 42);
        assert_eq!(cards[1].get_card(), 10);

        println!("{:?}", game_lost_zone);
    }
}
