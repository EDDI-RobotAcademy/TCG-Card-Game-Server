use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;

pub trait GameCardActiveSkillHandler: Send {
    unsafe fn summary_active_skill(&self) -> SummaryActiveSkillEffect;
}