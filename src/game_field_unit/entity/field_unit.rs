#[derive(Debug)]
pub struct FieldUnit {
    field_unit_card: i32,
}

impl FieldUnit {
    pub fn new(field_unit_card: i32) -> FieldUnit {
        FieldUnit { field_unit_card }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_unit_creation() {
        let field_unit_card = FieldUnit::new(5);
        assert_eq!(field_unit_card.get_card(), 5);
        println!("Test passed: FieldUnit creation and getter");
    }
}
