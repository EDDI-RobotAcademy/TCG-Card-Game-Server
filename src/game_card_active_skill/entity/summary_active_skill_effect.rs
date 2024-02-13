use crate::game_card_active_skill::entity::skill_type::SkillType;

pub struct SummaryActiveSkillEffect {
    // TODO: Required Energy Type and count 고려해야함
    skill_type: SkillType,
    skill_damage: i32,
    target_unit_count: i32
}

impl SummaryActiveSkillEffect {
    pub fn new(skill_type: SkillType,
               skill_damage: i32,
               target_unit_count: i32) -> Self {

        SummaryActiveSkillEffect {
            skill_type,
            skill_damage,
            target_unit_count
        }
    }

    pub fn get_skill_type(&self) -> &SkillType {
        &self.skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }

    pub fn get_target_unit_count(&self) -> i32 {
        self.target_unit_count
    }
}
