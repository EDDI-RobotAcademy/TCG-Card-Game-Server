use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_field_energy::service::request::check_field_energy_enough_to_use_request::CheckFieldEnergyEnoughToUseRequest;
use crate::game_field_energy::service::request::get_current_field_energy_request::GetCurrentFieldEnergyRequest;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_to_my_specific_unit_request::NoticeUseFieldEnergyToMySpecificUnitRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::GenerateMyFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::GenerateMySpecificUnitEnergyDataRequest;

#[derive(Debug)]
pub struct AttachFieldEnergyToFieldUnitRequestForm {
    session_id: String,
    unit_index: String,
    energy_race: String,
    quantity: String,
}

impl AttachFieldEnergyToFieldUnitRequestForm {
    pub fn new(session_id: String,
               unit_index: String,
               energy_race: String,
               quantity: String) -> Self {
        AttachFieldEnergyToFieldUnitRequestForm {
            session_id: session_id.to_string(),
            unit_index: unit_index.to_string(),
            energy_race: energy_race.to_string(),
            quantity: quantity.to_string()
        }
    }

    pub fn get_session_id(&self) -> &str { &self.session_id }

    pub fn get_unit_index(&self) -> &str { &self.unit_index }

    pub fn get_energy_race(&self) -> &str { &self.energy_race }

    pub fn get_quantity(&self) -> &str { &self.quantity }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_check_field_energy_enough_to_use_request(
        &self,
        account_unique_id: i32,
        will_be_used_amount: i32) -> CheckFieldEnergyEnoughToUseRequest {

        CheckFieldEnergyEnoughToUseRequest::new(
            account_unique_id,
            will_be_used_amount)
    }

    pub fn to_attach_single_energy_to_unit_index_request(
        &self,
        account_unique_id: i32,
        unit_card_index: i32,
        race_enum: RaceEnum) -> AttachSingleEnergyToUnitIndexRequest {

        AttachSingleEnergyToUnitIndexRequest::new(
            account_unique_id,
            unit_card_index,
            race_enum)
    }

    pub fn to_attach_multiple_energy_to_unit_index_request(
        &self,
        account_unique_id: i32,
        unit_card_index: i32,
        race_enum: RaceEnum,
        quantity: i32) -> AttachMultipleEnergyToUnitIndexRequest {

        AttachMultipleEnergyToUnitIndexRequest::new(
            account_unique_id,
            unit_card_index,
            race_enum,
            quantity)
    }

    pub fn to_get_current_attached_energy_of_field_unit_by_index_request(
        &self,
        account_unique_id: i32,
        field_unit_index: i32,) -> GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {

        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest::new(
            account_unique_id,
            field_unit_index)
    }

    pub fn to_remove_field_energy_with_amount_request(
        &self,
        account_unique_id: i32,
        amount: i32) -> RemoveFieldEnergyWithAmountRequest {

        RemoveFieldEnergyWithAmountRequest::new(
            account_unique_id,
            amount)
    }

    pub fn to_get_current_field_energy_request(
        &self,
        account_unique_id: i32) -> GetCurrentFieldEnergyRequest {

        GetCurrentFieldEnergyRequest::new(
            account_unique_id)
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_generate_my_field_energy_data_request(
        &self,
        remaining_field_energy: i32) -> GenerateMyFieldEnergyDataRequest {

        GenerateMyFieldEnergyDataRequest::new(
            remaining_field_energy)
    }

    pub fn to_generate_my_specific_unit_energy_data_request(
        &self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap) -> GenerateMySpecificUnitEnergyDataRequest {

        GenerateMySpecificUnitEnergyDataRequest::new(
            unit_index,
            updated_unit_energy_map)
    }

    pub fn to_notice_use_field_energy_to_my_specific_unit_request(
        &self,
        opponent_unique_id: i32,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> NoticeUseFieldEnergyToMySpecificUnitRequest {

        NoticeUseFieldEnergyToMySpecificUnitRequest::new(
            opponent_unique_id,
            player_field_energy_map_for_notice,
            player_field_unit_energy_map_for_notice)
    }
}