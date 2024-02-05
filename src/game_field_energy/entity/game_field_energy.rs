#[derive(PartialEq, Debug)]
pub struct GameFieldEnergy {
    energy_count: i32,
}

impl GameFieldEnergy {
    pub fn new(energy_count: i32) -> Self {
        GameFieldEnergy { energy_count }
    }
    pub fn get_energy_count(&self) -> i32 {
        self.energy_count
    }
    pub fn add_energy_count(&mut self) { self.energy_count += 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_energy() {
        let energy = GameFieldEnergy::new(3);
        assert_eq!(energy.get_energy_count(), 3);
        println!("Constructor test passed. Energy count: {}", energy.get_energy_count());

        let energy = GameFieldEnergy { energy_count: 7 };
        assert_eq!(energy.get_energy_count(), 7);
        println!("Getter test passed. Energy count: {}", energy.get_energy_count());
    }
}
