#[derive(Debug)]
pub struct GameHandCard {
    hand_card: i32,
}

impl GameHandCard {
    pub fn new(hand_card: i32) -> GameHandCard {
        GameHandCard { hand_card }
    }

    pub fn get_card(&self) -> i32 {
        self.hand_card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_card_creation() {
        let hand_card = GameHandCard::new(123);
        assert_eq!(hand_card.get_card(), 123);
        println!("Test passed: HandCard creation and getter");
    }
}
