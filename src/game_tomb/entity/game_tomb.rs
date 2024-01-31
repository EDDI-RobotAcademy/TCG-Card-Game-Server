use crate::game_tomb::entity::tomb_card::TombCard;
use crate::game_tomb::entity::tomb_card_list::TombCardList;

#[derive(Debug)]
pub struct GameTomb {
    game_tomb: TombCardList,
}

impl GameTomb {
    pub fn new() -> GameTomb {
        GameTomb {
            game_tomb: TombCardList::new(),
        }
    }

    pub fn add_tomb_card(&mut self, card_id: i32) {
        let tomb_card = TombCard::new(card_id);
        self.game_tomb.add_tomb_card(tomb_card);
    }

    pub fn get_tomb_card_list(&self) -> &TombCardList {
        &self.game_tomb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_lost_zone() {
        let mut game_tomb = GameTomb::new();

        game_tomb.add_tomb_card(77);
        game_tomb.add_tomb_card(3);

        let card_list = game_tomb.get_tomb_card_list();
        let cards = card_list.get_tomb_card_list();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].get_card(), 77);
        assert_eq!(cards[1].get_card(), 3);

        println!("{:?}", game_tomb);
    }
}
