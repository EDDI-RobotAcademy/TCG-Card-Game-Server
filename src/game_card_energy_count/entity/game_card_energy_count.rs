#[derive(Debug)]
pub struct GameCardEnergyCount {
    energy_count: i32
}

impl GameCardEnergyCount {
    pub fn new() -> Self { GameCardEnergyCount { energy_count: 0 } }
    pub fn add_energy_to_card(&mut self) { self.energy_count += 1 }
    pub fn get_energy_count(&self) -> i32 { self.energy_count }
}