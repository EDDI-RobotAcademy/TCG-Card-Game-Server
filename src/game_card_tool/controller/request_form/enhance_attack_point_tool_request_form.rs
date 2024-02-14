use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_tool::service::request::summarize_tool_card_effect_request::SummarizeToolCardEffectRequest;
use crate::game_hand::service::request::use_game_hand_tool_card_request::UseGameHandToolCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_tool_card_request::IsItToolCardRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_boost_card_request::NotifyToOpponentYouUseEnergyBoostCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct EnhanceAttackPointToolRequestForm {
    session_id: String,
    unit_index_number: String,
    tool_card_id: String,
}

impl EnhanceAttackPointToolRequestForm {
    pub fn new(session_id: String, unit_index_number: String, tool_card_id: String) -> Self {
        EnhanceAttackPointToolRequestForm {
            session_id: session_id.to_string(),
            unit_index_number: unit_index_number.to_string(),
            tool_card_id: tool_card_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn get_unit_index_number(&self) -> &str {
        &self.unit_index_number
    }
    pub fn get_tool_card_id(&self) -> &str {
        &self.tool_card_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }
    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, tool_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, tool_card_number)
    }
    pub fn to_can_use_card_request(&self, account_unique_id: i32, tool_card_number: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, tool_card_number)
    }
    pub fn to_is_it_tool_card_request(&self, tool_card_number: i32) -> IsItToolCardRequest {
        IsItToolCardRequest::new(tool_card_number)
    }
    pub fn to_use_game_hand_tool_card_request(&self, account_unique_id: i32, tool_card_number: i32) -> UseGameHandToolCardRequest {
        UseGameHandToolCardRequest::new(
            account_unique_id, tool_card_number)
    }
    // TODO: 유닛 카드가 무덤으로 이동할 때 함께 무덤으로 이동이 필요함 (유닛 카드 속성에 도구 카드 번호를 부여하는 식으로)
    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }
    pub fn to_summarize_tool_card_effect_request(&self, tool_card_id: i32) -> SummarizeToolCardEffectRequest {
        SummarizeToolCardEffectRequest::new(tool_card_id)
    }
    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }
// TODO: enhance attack point 기능에 부합하게 내용 수정이 필요 (필드에 드로우된 카드의 속성값을 가져오는 선행 작업이 필요함)
    pub fn to_notify_to_opponent_you_use_enhance_attack_point_card_request(
        &self, opponent_unique_id: i32,
        unit_card_index: i32,
        usage_hand_card_id: i32,
        boosting_energy_count: i32,
        boosting_energy_card_id: i32) -> NotifyToOpponentYouUseEnergyBoostCardRequest {

        NotifyToOpponentYouUseEnergyBoostCardRequest::new(
            opponent_unique_id, unit_card_index, usage_hand_card_id, boosting_energy_count, boosting_energy_card_id)
    }
}