#[derive(Debug)]
pub struct DeckCard {
    card: i32,
}

impl DeckCard {
    pub fn new(card: i32) -> DeckCard {
        DeckCard { card }
    }

    pub fn get_card(&self) -> i32 {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_card_creation() {
        let card = DeckCard::new(42);
        assert_eq!(card.get_card(), 42);
        println!("Test passed: DeckCard creation and getter");
    }
}
