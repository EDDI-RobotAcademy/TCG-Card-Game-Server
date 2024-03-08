use std::collections::HashMap;
use crate::action_waiting_timer::service::request::action_waiting_timer_request::ActionWaitingTimerRequest;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_passive_skill::service::request::summary_turn_start_passive_skill_effect_request::SummaryTurnStartPassiveSkillEffectRequest;
use crate::game_card_support_usage_counter::service::request::reset_support_card_usage_count_request::ResetSupportCardUsageCountRequest;
use crate::game_card_unit::service::request::summary_unit_card_passive_default_request::SummaryUnitCardPassiveDefaultRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_field_unit::service::request::apply_status_effect_damage_iteratively_request::ApplyStatusEffectDamageIterativelyRequest;
use crate::game_round::service::request::next_game_turn_request::NextGameRoundRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::request::get_current_field_energy_request::GetCurrentFieldEnergyRequest;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::service::request::acquire_harmful_status_effect_of_all_unit_request::AcquireHarmfulStatusEffectOfAllUnitRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_all_field_unit_request::GetCurrentHealthPointOfAllFieldUnitRequest;
use crate::game_field_unit::service::request::get_game_field_unit_card_of_account_uique_id_request::GetGameFieldUnitCardOfAccountUniqueIdRequest;
use crate::game_field_unit::service::request::judge_death_of_every_unit_request::JudgeDeathOfEveryUnitRequest;
use crate::game_field_unit::service::request::reset_all_passive_of_unit_request::ResetAllPassiveOfUnitRequest;
use crate::game_field_unit::service::request::reset_turn_action_of_all_field_unit_request::ResetTurnActionOfAllFieldUnitRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::Death;
use crate::game_main_character::service::request::set_main_character_as_death_request::SetMainCharacterAsDeathRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::add_dead_unit_list_to_tomb_request::AddDeadUnitListToTombRequest;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_my_turn_end_request::NoticeMyTurnEndRequest;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex::You;
use crate::ui_data_generator::service::request::generate_draw_opponent_deck_data_request::GenerateDrawOpponentDeckDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_death_data_request::GenerateMyMultipleUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_harmful_effect_data_request::GenerateMyMultipleUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_health_point_data_request::GenerateMyMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_field_energy_data_request::GenerateOpponentFieldEnergyDataRequest;

#[derive(Debug)]
pub struct TurnEndRequestForm {
    session_id: String,

}

impl TurnEndRequestForm {
    pub fn new(session_id: String,) -> Self {
        TurnEndRequestForm {
            session_id,

        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_is_this_your_turn_request(&self, account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_apply_status_effect_damage_iteratively_request(&self, account_unique_id: i32) -> ApplyStatusEffectDamageIterativelyRequest {
        ApplyStatusEffectDamageIterativelyRequest::new(account_unique_id)
    }

    pub fn to_judge_death_of_every_unit_request(&self, account_unique_id: i32) -> JudgeDeathOfEveryUnitRequest {
        JudgeDeathOfEveryUnitRequest::new(account_unique_id)
    }

    pub fn to_add_dead_unit_list_to_tomb_request(&self, account_unique_id: i32, dead_unit_list: Vec<i32>) -> AddDeadUnitListToTombRequest {
        AddDeadUnitListToTombRequest::new(account_unique_id, dead_unit_list)
    }

    pub fn to_game_field_unit_card_of_account_unique_id_request(&self, account_unique_id: i32) -> GetGameFieldUnitCardOfAccountUniqueIdRequest {
        GetGameFieldUnitCardOfAccountUniqueIdRequest::new(account_unique_id)
    }

    pub fn to_reset_turn_action_of_all_field_unit_request(&self, account_unique_id: i32) -> ResetTurnActionOfAllFieldUnitRequest {
        ResetTurnActionOfAllFieldUnitRequest::new(account_unique_id)
    }

    pub fn to_reset_support_card_usage_count_request(&self, account_unique_id: i32) -> ResetSupportCardUsageCountRequest {
        ResetSupportCardUsageCountRequest::new(account_unique_id)
    }

    pub fn to_next_turn_request(&self, account_unique_id: i32) -> NextTurnRequest {
        NextTurnRequest::new(account_unique_id)
    }

    pub fn to_next_round_request(&self) -> NextGameRoundRequest {
        NextGameRoundRequest::new(self.session_id.clone())
    }

    pub fn to_draw_cards_from_deck_request(&self, opponent_account_unique_id: i32) -> DrawCardsFromDeckRequest {
        DrawCardsFromDeckRequest::new(opponent_account_unique_id, 1)
    }

    pub fn to_add_card_list_to_hand_request(&self, account_unique_id: i32, card_list: Vec<i32>) -> AddCardListToHandRequest {
        AddCardListToHandRequest::new(account_unique_id, card_list)
    }

    pub fn to_add_field_energy_request(&self, opponent_account_unique_id: i32) -> AddFieldEnergyWithAmountRequest {
        AddFieldEnergyWithAmountRequest::new(opponent_account_unique_id, 1)
    }

    pub fn to_get_current_field_energy_request(
        &self,
        opponent_account_unique_id: i32) -> GetCurrentFieldEnergyRequest {

        GetCurrentFieldEnergyRequest::new(
            opponent_account_unique_id)
    }

    pub fn to_get_current_health_point_of_all_field_unit_request(
        &self,
        account_unique_id: i32
    ) -> GetCurrentHealthPointOfAllFieldUnitRequest {

        GetCurrentHealthPointOfAllFieldUnitRequest::new(
            account_unique_id)
    }

    pub fn to_acquire_harmful_status_effect_of_all_unit_request(
        &self,
        account_unique_id: i32) -> AcquireHarmfulStatusEffectOfAllUnitRequest {

        AcquireHarmfulStatusEffectOfAllUnitRequest::new(
            account_unique_id)
    }

    pub fn to_get_game_field_unit_card_of_account_unique_id_request(
        &self,
        account_unique_id: i32) -> GetGameFieldUnitCardOfAccountUniqueIdRequest {

        GetGameFieldUnitCardOfAccountUniqueIdRequest::new(
            account_unique_id)
    }

    pub fn to_summary_unit_card_passive_default_request(
        &self,
        unit_card_it: i32) -> SummaryUnitCardPassiveDefaultRequest {

        SummaryUnitCardPassiveDefaultRequest::new(
            unit_card_it)
    }

    pub fn to_reset_all_passive_of_unit(
        &self,
        account_unique_id: i32,
        unit_card_index: i32,
        passive_default_list: Vec<bool>) -> ResetAllPassiveOfUnitRequest {

        ResetAllPassiveOfUnitRequest::new(
            account_unique_id,
            unit_card_index,
            passive_default_list)
    }

    pub fn to_action_waiting_timer(
        &self,
        opponent_account_unique_id: i32) -> ActionWaitingTimerRequest {

        ActionWaitingTimerRequest::new(
            opponent_account_unique_id)
    }

    pub fn to_set_main_character_as_death_request(
        &self,
        account_unique_id: i32) -> SetMainCharacterAsDeathRequest {

        SetMainCharacterAsDeathRequest::new(
            account_unique_id)
    }

    pub fn to_set_game_winner_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32,) -> CheckMainCharacterRequest {

        CheckMainCharacterRequest::new(
            account_unique_id,
            opponent_unique_id)
    }

    pub fn to_summary_turn_start_passive_skill_effect_request(
        &self,
        unit_card_id: i32) -> SummaryTurnStartPassiveSkillEffectRequest {

        SummaryTurnStartPassiveSkillEffectRequest::new(
            unit_card_id)
    }

    pub fn to_generate_draw_opponent_deck_data(
        &self,
        drawn_card_list: Vec<i32>
    ) -> GenerateDrawOpponentDeckDataRequest {

        GenerateDrawOpponentDeckDataRequest::new(
            drawn_card_list)
    }

    pub fn to_generate_opponent_field_energy_data_request(
        &self,
        remaining_field_energy: i32) -> GenerateOpponentFieldEnergyDataRequest {

        GenerateOpponentFieldEnergyDataRequest::new(
            remaining_field_energy)
    }

    pub fn to_generate_my_multiple_unit_health_point_data_request(
        &self,
        my_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> GenerateMyMultipleUnitHealthPointDataRequest {

        GenerateMyMultipleUnitHealthPointDataRequest::new(
            my_unit_health_point_tuple_list)
    }

    pub fn to_generate_my_multiple_unit_harmful_effect_data_request(
        &self,
        my_unit_harmful_status_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> GenerateMyMultipleUnitHarmfulEffectDataRequest {

        GenerateMyMultipleUnitHarmfulEffectDataRequest::new(
            my_unit_harmful_status_tuple_list)
    }

    pub fn to_generate_my_multiple_unit_death_data_request(
        &self,
        my_dead_unit_index_list: Vec<i32>
    ) -> GenerateMyMultipleUnitDeathDataRequest {

        GenerateMyMultipleUnitDeathDataRequest::new(
            my_dead_unit_index_list)
    }

    pub fn to_notice_my_turn_end_request(
        &self,
        opponent_unique_id: i32,
        player_drawn_card_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        unit_index_turn_start_passive_list_map: HashMap<i32, Vec<i32>>,
    ) -> NoticeMyTurnEndRequest {

        NoticeMyTurnEndRequest::new(
            opponent_unique_id,
            player_drawn_card_list_map_for_notice,
            player_field_energy_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_harmful_effect_map_for_notice,
            player_field_unit_death_map_for_notice,
            HashMap::new(),
            unit_index_turn_start_passive_list_map)
    }

    pub fn to_notice_my_turn_end_in_case_of_no_more_opponent_deck(
        &self,
        opponent_unique_id: i32,) -> NoticeMyTurnEndRequest {

        let mut map_only_for_death_by_no_more_opponent_deck_card = HashMap::new();
        map_only_for_death_by_no_more_opponent_deck_card.insert(You, Death);

        NoticeMyTurnEndRequest::new(
            opponent_unique_id,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            map_only_for_death_by_no_more_opponent_deck_card,
            HashMap::new())
    }
}