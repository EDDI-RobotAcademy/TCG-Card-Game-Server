#[derive(Debug, Clone)]
pub struct GameFieldUnitCard {
    field_unit_card: i32,
    attached_energy: i32,
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32) -> GameFieldUnitCard {
        GameFieldUnitCard {
            field_unit_card,
            attached_energy: 0,
        }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }

    pub fn get_attached_energy(&self) -> i32 {
        self.attached_energy
    }

    pub fn attach_energy(&mut self) {
        self.attached_energy += 1;
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

    #[test]
    fn test_attach_energy() {
        let mut game_field_unit_card = GameFieldUnitCard::new(5);
        assert_eq!(game_field_unit_card.get_card(), 5);
        assert_eq!(game_field_unit_card.get_attached_energy(), 0);

        game_field_unit_card.attach_energy();
        assert_eq!(game_field_unit_card.get_attached_energy(), 1);

        println!("{:?}", game_field_unit_card);

        println!("Test passed: FieldUnit creation, getter, attach_energy, and print_state");
    }
}
