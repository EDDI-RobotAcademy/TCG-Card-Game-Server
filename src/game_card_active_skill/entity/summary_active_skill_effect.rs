use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;
use crate::game_card_active_skill::entity::required_energy::RequiredEnergy;

pub struct SummaryActiveSkillEffect {
    required_energy: RequiredEnergy,
    active_skill_type: ActiveSkillType,
    skill_damage: i32,
}

impl SummaryActiveSkillEffect {
    pub fn new(required_energy_race: RaceEnum,
               required_energy_count: i32,
               active_skill_type: ActiveSkillType,
               skill_damage: i32) -> Self {

        SummaryActiveSkillEffect {
            required_energy: RequiredEnergy::new(required_energy_race, required_energy_count),
            active_skill_type,
            skill_damage,
        }
    }

    pub fn get_required_energy_for_active_skill(&self) -> &RequiredEnergy { &self.required_energy }

    pub fn get_skill_type(&self) -> &ActiveSkillType {
        &self.active_skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }
}
