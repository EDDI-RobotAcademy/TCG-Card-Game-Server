use async_trait::async_trait;
use crate::game_card_active_skill::controller::request_form::non_targeting_active_skill_request_form::NonTargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::request_form::targeting_active_skill_request_form::TargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::request_form::targeting_attack_active_skill_to_game_main_character_request_form::TargetingAttackActiveSkillToGameMainCharacterRequestForm;
use crate::game_card_active_skill::controller::response_form::non_targeting_active_skill_response_form::NonTargetingActiveSkillResponseForm;
use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;
use crate::game_card_active_skill::controller::response_form::targeting_attack_active_skill_to_game_main_character_response_form::TargetingAttackActiveSkillToGameMainCharacterResponseForm;

#[async_trait]
pub trait GameCardActiveSkillController {
    async fn request_targeting_active_skill(
        &self, targeting_active_skill_request_form: TargetingActiveSkillRequestForm)
        -> TargetingActiveSkillResponseForm;

    async fn request_non_targeting_active_skill(
        &self, non_targeting_active_skill_request_form: NonTargetingActiveSkillRequestForm)
        -> NonTargetingActiveSkillResponseForm;
    async fn request_targeting_active_skill_to_game_main_character(
        &self, targeting_attack_active_skill_to_game_main_character_request_form: TargetingAttackActiveSkillToGameMainCharacterRequestForm)
        -> TargetingAttackActiveSkillToGameMainCharacterResponseForm;
}
