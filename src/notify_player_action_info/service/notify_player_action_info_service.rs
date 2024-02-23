use async_trait::async_trait;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_by_using_hand_card_request::NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_draw_card_by_using_hand_card_request::NoticeDrawCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_by_using_hand_card_response::NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_draw_card_by_using_hand_card_response::NoticeDrawCardByUsingHandCardResponse;

#[async_trait]
pub trait NotifyPlayerActionInfoService {
    async fn notice_boost_energy_to_specific_unit_by_using_hand_card(
        &mut self, notice_boost_energy_to_specific_unit_by_using_hand_card_request: NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse;
    async fn notice_draw_card_by_using_hand_card(
        &mut self, notice_draw_card_by_using_hand_card_request: NoticeDrawCardByUsingHandCardRequest)
        -> NoticeDrawCardByUsingHandCardResponse;
}