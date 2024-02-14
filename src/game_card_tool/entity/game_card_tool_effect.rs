use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::required_energy::RequiredEnergy;

#[derive(PartialEq)]
pub struct GameCardToolEffect {
    required_energy: RequiredEnergy,
    enhance_attack_point: i32,
}

impl GameCardToolEffect {
    pub fn new(required_energy_race: RaceEnum, required_energy_count: i32) -> Self {

        GameCardToolEffect {
            required_energy: RequiredEnergy::new(required_energy_race, required_energy_count),
            enhance_attack_point: -1,
        }
    }

    // Enhance attack point
    pub fn get_enhance_attack_point(&self) -> i32 { self.enhance_attack_point }
    pub fn set_enhance_attack_point(&mut self, enhance_attack_point: i32) {
        self.enhance_attack_point = enhance_attack_point;
    }
}