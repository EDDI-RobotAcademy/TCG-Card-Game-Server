use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::message::false_message_enum::FalseMessage;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_extra_effect_data_response::GenerateMySpecificUnitExtraEffectDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachSpecialEnergyCardResponseForm {
    is_success: bool,
    false_message_enum: i32,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_field_unit_extra_effect_map: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
}

impl AttachSpecialEnergyCardResponseForm {
    pub fn new( is_success: bool,
                false_message_enum: i32,
                player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
                player_field_unit_extra_effect_map: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>
    ) -> Self {
        AttachSpecialEnergyCardResponseForm {
            is_success,
            false_message_enum,
            player_field_unit_energy_map,
            player_field_unit_extra_effect_map
        }
    }
    pub fn from_response(
        generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
        generate_my_specific_unit_energy_data_response: GenerateMySpecificUnitEnergyDataResponse,
        generate_my_specific_unit_extra_effect_data_response: GenerateMySpecificUnitExtraEffectDataResponse
    ) -> AttachSpecialEnergyCardResponseForm {

        AttachSpecialEnergyCardResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            -1,
            generate_my_specific_unit_energy_data_response.get_player_field_unit_energy_map_for_response().clone(),
            generate_my_specific_unit_extra_effect_data_response.get_player_field_unit_extra_effect_map_for_response().clone())
    }

    pub fn default() -> AttachSpecialEnergyCardResponseForm {

        AttachSpecialEnergyCardResponseForm::new(
            false,
            -1,
            HashMap::new(),
            HashMap::new())
    }
    pub fn from_response_with_message(false_message: FalseMessage) -> AttachSpecialEnergyCardResponseForm {

        AttachSpecialEnergyCardResponseForm::new(
            false,
            false_message as i32,
            HashMap::new(),
            HashMap::new())
    }
}
