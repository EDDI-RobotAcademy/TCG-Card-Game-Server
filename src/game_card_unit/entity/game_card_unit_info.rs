use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

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
    // TODO: 병종과 관련한 enum 값도 정리가 필요함
    military_occupational_specialty: i32,
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
                military_occupational_specialty
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

    pub fn get_first_passive_skill(&self) -> bool {
        self.first_passive_skill
    }

    pub fn get_second_passive_skill(&self) -> bool {
        self.second_passive_skill
    }

    pub fn get_third_passive_skill(&self) -> bool {
        self.third_passive_skill
    }
}