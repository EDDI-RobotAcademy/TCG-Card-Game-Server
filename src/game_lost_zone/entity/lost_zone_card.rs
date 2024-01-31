#[derive(Debug)]
pub struct LostZoneCard {
    card: i32
}

impl LostZoneCard {
    pub fn new(card: i32) -> LostZoneCard {
        LostZoneCard { card }
    }

    pub fn get_card(&self) -> i32 {
        self.card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lost_zone_card() {
        let lost_zone_card = LostZoneCard::new(42);

        assert_eq!(lost_zone_card.get_card(), 42);

        println!("{:?}", lost_zone_card);
    }
}
