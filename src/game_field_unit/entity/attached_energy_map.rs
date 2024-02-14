use std::collections::HashMap;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

#[derive(Debug, Clone)]
pub struct AttachedEnergyMap {
    energy_map: HashMap<RaceEnumValue, i32>,
}

impl AttachedEnergyMap {
    pub fn new() -> AttachedEnergyMap {
        AttachedEnergyMap {
            energy_map: HashMap::new(),
        }
    }

    pub fn add_energy(&mut self, race: RaceEnumValue, quantity: i32) {
        *self.energy_map.entry(race).or_insert(0) += quantity;
    }

    pub fn get_energy_quantity(&self, race: &RaceEnumValue) -> Option<&i32> {
        self.energy_map.get(race)
    }

    pub fn remove_energy(&mut self, race: &RaceEnumValue, quantity: i32) {
        if let Some(energy) = self.energy_map.get_mut(race) {
            for _ in 0..quantity {
                *energy -= 1;
                if *energy == 0 {
                    self.energy_map.remove_entry(race);
                    break
                }
            }
        }
    }

    pub fn get_all_energy(&self) -> &HashMap<RaceEnumValue, i32> {
        &self.energy_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attached_energy_map_creation() {
        let attached_energy_map = AttachedEnergyMap::new();
        assert_eq!(attached_energy_map.get_all_energy().len(), 0);
        println!("Test passed: AttachedEnergyMap creation and getter");
    }

    #[test]
    fn test_add_energy() {
        let mut attached_energy_map = AttachedEnergyMap::new();
        assert_eq!(attached_energy_map.get_all_energy().len(), 0);

        attached_energy_map.add_energy(RaceEnumValue::Undead, 3);
        assert_eq!(attached_energy_map.get_all_energy().len(), 1);
        assert_eq!(*attached_energy_map.get_energy_quantity(&RaceEnumValue::Undead).unwrap(), 3);

        attached_energy_map.add_energy(RaceEnumValue::Undead, 3);
        assert_eq!(*attached_energy_map.get_energy_quantity(&RaceEnumValue::Undead).unwrap(), 6);

        attached_energy_map.add_energy(RaceEnumValue::Human, 5);
        assert_eq!(attached_energy_map.get_all_energy().len(), 2);
        assert_eq!(*attached_energy_map.get_energy_quantity(&RaceEnumValue::Human).unwrap(), 5);

        println!("{:?}", attached_energy_map);

        println!("Test passed: AttachedEnergyMap add_energy and getter");
    }

    #[test]
    fn test_remove_energy() {
        let mut attached_energy_map = AttachedEnergyMap::new();

        attached_energy_map.add_energy(RaceEnumValue::Human, 5);
        attached_energy_map.add_energy(RaceEnumValue::Undead, 3);

        attached_energy_map.remove_energy(&RaceEnumValue::Human, 2);
        attached_energy_map.remove_energy(&RaceEnumValue::Undead, 1);

        println!("{:?}", attached_energy_map);

        attached_energy_map.remove_energy(&RaceEnumValue::Undead, 2);

        println!("{:?}", attached_energy_map);
    }
}