#[derive(Debug, Clone)]
pub struct GameDeckCard {
    card: i32,
}

impl GameDeckCard {
    pub fn new(card: i32) -> GameDeckCard {
        GameDeckCard { card }
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
        let card = GameDeckCard::new(42);
        assert_eq!(card.get_card(), 42);
        println!("Test passed: DeckCard creation and getter");
    }
}
