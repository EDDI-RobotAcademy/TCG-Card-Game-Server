use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_unit::entity::passive_status::PassiveStatus;

#[derive(Debug)]
pub struct GameCardUnitInfo {
    race: RaceEnum,
    grade: GradeEnum,
    attack_point: i32,
    health_point: i32,
    attack_required_energy: i32,
    first_active_skill_required_energy: i32,
    second_active_skill_required_energy: i32,
    third_active_skill_required_energy: i32,
    first_passive_skill: bool,
    second_passive_skill: bool,
    third_passive_skill: bool,
    military_occupational_specialty: i32,
    passive_status_list: Vec<PassiveStatus>
}

impl GameCardUnitInfo {
    pub fn new(
        race: RaceEnum,
        grade: GradeEnum,
        attack_point: i32,
        health_point: i32,
        attack_required_energy: i32,
        first_active_skill_required_energy: i32,
        second_active_skill_required_energy: i32,
        third_active_skill_required_energy: i32,
        first_passive_skill: bool,
        second_passive_skill: bool,
        third_passive_skill: bool,
        military_occupational_specialty: i32) -> Self {

            Self {
                race,
                grade,
                attack_point,
                health_point,
                attack_required_energy,
                first_active_skill_required_energy,
                second_active_skill_required_energy,
                third_active_skill_required_energy,
                first_passive_skill,
                second_passive_skill,
                third_passive_skill,
                military_occupational_specialty,
                passive_status_list: Vec::new()
            }
    }

    pub fn get_race(&self) -> RaceEnum {
        self.race
    }

    pub fn get_grade(&self) -> GradeEnum {
        self.grade
    }

    pub fn get_attack_point(&self) -> i32 {
        self.attack_point
    }

    pub fn get_health_point(&self) -> i32 {
        self.health_point
    }

    pub fn get_attack_required_energy(&self) -> i32 {
        self.attack_required_energy
    }

    pub fn get_first_active_skill_required_energy(&self) -> i32 {
        self.first_active_skill_required_energy
    }

    pub fn get_second_active_skill_required_energy(&self) -> i32 {
        self.second_active_skill_required_energy
    }

    pub fn get_third_active_skill_required_energy(&self) -> i32 {
        self.third_active_skill_required_energy
    }

    pub fn get_military_occupational_specialty(&self) -> i32 {
        self.military_occupational_specialty
    }

    pub fn has_first_passive_skill(&self) -> bool {
        self.first_passive_skill
    }

    pub fn has_second_passive_skill(&self) -> bool {
        self.second_passive_skill
    }

    pub fn has_third_passive_skill(&self) -> bool {
        self.third_passive_skill
    }

    pub fn set_passive_status_list(&mut self, passive_status_list: Vec<PassiveStatus>) {
        self.passive_status_list = passive_status_list;
    }
    pub fn get_passive_status_list(&self) -> &Vec<PassiveStatus> { &self.passive_status_list }
}