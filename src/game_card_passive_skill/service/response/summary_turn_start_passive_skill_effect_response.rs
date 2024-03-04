use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;

#[derive(Debug, PartialEq)]
pub struct SummaryTurnStartPassiveSkillEffectResponse {
    passive_skill_effect_list: Vec<SummaryPassiveSkillEffect>
}

impl SummaryTurnStartPassiveSkillEffectResponse {
    pub fn new(
        first_passive_skill_effect: SummaryPassiveSkillEffect,
        second_passive_skill_effect: SummaryPassiveSkillEffect,
        third_passive_skill_effect: SummaryPassiveSkillEffect
    ) -> Self {

        let passive_skill_effect_list = vec![
            first_passive_skill_effect,
            second_passive_skill_effect,
            third_passive_skill_effect,
        ];

        SummaryTurnStartPassiveSkillEffectResponse {
            passive_skill_effect_list,
        }
    }

    pub fn get_passive_skill_effect_list(self) -> Vec<SummaryPassiveSkillEffect> {
        self.passive_skill_effect_list
    }

    pub fn is_empty(&self) -> bool {
        self.passive_skill_effect_list.is_empty()
    }
}
