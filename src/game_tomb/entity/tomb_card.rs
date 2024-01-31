#[derive(Debug)]
pub struct TombCard {
    card: i32
}

impl TombCard {
    pub fn new(card: i32) -> TombCard {
        TombCard { card }
    }

    pub fn get_card(&self) -> i32 {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tomb_card() {
        let tomb_card = TombCard::new(42);

        assert_eq!(tomb_card.get_card(), 42);

        println!("{:?}", tomb_card);
    }
}
