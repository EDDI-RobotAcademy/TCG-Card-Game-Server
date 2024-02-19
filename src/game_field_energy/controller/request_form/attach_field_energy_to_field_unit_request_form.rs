use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_field_energy::service::request::check_field_energy_enough_to_use_request::CheckFieldEnergyEnoughToUseRequest;
use crate::game_field_energy::service::request::get_current_field_energy_request::GetCurrentFieldEnergyRequest;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_unit_index_request::AttachMultipleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::attach_single_energy_to_unit_index_request::AttachSingleEnergyToUnitIndexRequest;
use crate::game_field_unit::service::request::get_current_attached_energy_of_field_unit_by_index_request::GetCurrentAttachedEnergyOfFieldUnitByIndexRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_attached_field_energy_to_field_unit_request::NotifyOpponentYouAttachedFieldEnergyRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct AttachFieldEnergyToFieldUnitRequestForm {
    session_id: String,
    unit_index: String,
    energy_race: String,
    quantity: String,
}

impl AttachFieldEnergyToFieldUnitRequestForm {
    pub fn new(session_id: &str,
               unit_index: &str,
               energy_race: &str,
               quantity: &str) -> Self {
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

    pub fn to_check_field_energy_enough_to_use_request(&self,
                                                       account_unique_id: i32,
                                                       will_be_used_amount: i32) -> CheckFieldEnergyEnoughToUseRequest {
        CheckFieldEnergyEnoughToUseRequest::new(account_unique_id,
                                                will_be_used_amount)
    }

    pub fn to_attach_single_energy_to_unit_index_request(&self,
                                                         account_unique_id: i32,
                                                         unit_card_index: i32,
                                                         race_enum: RaceEnum) -> AttachSingleEnergyToUnitIndexRequest {
        AttachSingleEnergyToUnitIndexRequest::new(account_unique_id,
                                                  unit_card_index,
                                                  race_enum)
    }

    pub fn to_attach_multiple_energy_to_unit_index_request(&self,
                                                           account_unique_id: i32,
                                                           unit_card_index: i32,
                                                           race_enum: RaceEnum,
                                                           quantity: i32) -> AttachMultipleEnergyToUnitIndexRequest {
        AttachMultipleEnergyToUnitIndexRequest::new(account_unique_id,
                                                    unit_card_index,
                                                    race_enum,
                                                    quantity)
    }

    pub fn to_get_current_attached_energy_of_field_unit_by_index_request(&self,
                                                                         account_unique_id: i32,
                                                                         unit_card_index: i32,
                                                                         race_enum: RaceEnum) -> GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {
        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest::new(account_unique_id,
                                                               unit_card_index,
                                                               race_enum)
    }

    pub fn to_remove_field_energy_with_amount_request(&self,
                                                      account_unique_id: i32,
                                                      amount: i32) -> RemoveFieldEnergyWithAmountRequest {
        RemoveFieldEnergyWithAmountRequest::new(account_unique_id,
                                                amount)
    }

    pub fn to_get_current_field_energy_request(&self, account_unique_id: i32) -> GetCurrentFieldEnergyRequest {
        GetCurrentFieldEnergyRequest::new(account_unique_id)
    }

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_notify_you_attach_field_energy_to_field_unit_request(&self,
                                                                   opponent_unique_id: i32,
                                                                   unit_card_index: i32,
                                                                   energy_race_enum: RaceEnum,
                                                                   energy_count: i32,
                                                                   current_unit_energy_count: i32,
                                                                   remaining_field_energy: i32) -> NotifyOpponentYouAttachedFieldEnergyRequest {
        let energy_race = energy_race_enum as i32;
        NotifyOpponentYouAttachedFieldEnergyRequest::new(opponent_unique_id,
                                                         unit_card_index,
                                                         energy_race,
                                                         energy_count,
                                                         current_unit_energy_count,
                                                         remaining_field_energy)
    }
}