use crate::game_field_unit::entity::field_unit::FieldUnit;

#[derive(Debug)]
pub struct FieldUnitList {
    field_unit_card_list: Vec<FieldUnit>,
}

impl FieldUnitList {
    pub fn new() -> FieldUnitList {
        FieldUnitList { field_unit_card_list: Vec::new() }
    }

    pub fn add_field_unit(&mut self, card: FieldUnit) {
        self.field_unit_card_list.push(card);
    }

    pub fn get_all_field_unit_list(&self) -> &Vec<FieldUnit> {
        &self.field_unit_card_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_unit_list() {
        let mut field_unit_list = FieldUnitList::new();

        let field_unit1 = FieldUnit::new(3);
        let field_unit2 = FieldUnit::new(7);
        field_unit_list.add_field_unit(field_unit1);
        field_unit_list.add_field_unit(field_unit2);

        let field_unit_list = field_unit_list.get_all_field_unit_list();
        assert_eq!(field_unit_list.len(), 2);
        assert_eq!(field_unit_list[0].get_card(), 3);
        assert_eq!(field_unit_list[1].get_card(), 7);

        println!("{:?}", field_unit_list);
    }
}
