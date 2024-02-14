use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;

pub struct SummaryActiveSkillEffect {
    // TODO: Required Energy Type and count 고려해야함
    active_skill_type: ActiveSkillType,
    skill_damage: i32,
}

impl SummaryActiveSkillEffect {
    pub fn new(active_skill_type: ActiveSkillType,
               skill_damage: i32) -> Self {

        SummaryActiveSkillEffect {
            active_skill_type,
            skill_damage,
        }
    }

    pub fn get_skill_type(&self) -> &ActiveSkillType {
        &self.active_skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }
}
