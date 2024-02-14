use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;

#[derive(Debug)]
pub struct ApplyPassiveSkillListRequest {
    opponent_unique_id: i32,
    passive_skill_list: Vec<SummaryPassiveSkillEffect>
}

impl ApplyPassiveSkillListRequest {
    pub fn new(opponent_unique_id: i32,
               passive_skill_list: Vec<SummaryPassiveSkillEffect>) -> Self {

        ApplyPassiveSkillListRequest {
            opponent_unique_id,
            passive_skill_list,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_passive_skill_list(&self) -> &Vec<SummaryPassiveSkillEffect> {
        &self.passive_skill_list
    }
}
