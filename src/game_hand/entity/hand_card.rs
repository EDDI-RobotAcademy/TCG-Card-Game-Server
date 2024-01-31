#[derive(Debug)]
pub struct HandCard {
    hand_card: i32,
}

impl HandCard {
    pub fn new(hand_card: i32) -> HandCard {
        HandCard { hand_card }
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
        let hand_card = HandCard::new(123);
        assert_eq!(hand_card.get_card(), 123);
        println!("Test passed: HandCard creation and getter");
    }
}
