#[derive(Debug)]
pub struct GameFieldUnitCard {
    field_unit_card: i32,
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32) -> GameFieldUnitCard {
        GameFieldUnitCard { field_unit_card }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_field_unit_card_creation() {
        let game_field_unit_card = GameFieldUnitCard::new(5);
        assert_eq!(game_field_unit_card.get_card(), 5);
        println!("Test passed: FieldUnit creation and getter");
    }
}
