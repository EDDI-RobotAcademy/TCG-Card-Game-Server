use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;

pub trait GameCardPassiveSkillHandler: Send {
    unsafe fn summary_passive_skill(&self) -> SummaryPassiveSkillEffect;
}
