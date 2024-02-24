use async_trait::async_trait;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_unit_by_using_hand_card_request::NoticeApplyDamageToEveryUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_by_using_hand_card_request::NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_specific_unit_by_using_hand_card_request::NoticeApplyDamageToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_attach_energy_to_specific_unit_by_using_hand_card_request::NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_draw_card_by_using_hand_card_request::NoticeDrawCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_unit_by_using_hand_card_request::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_remove_field_energy_by_using_hand_card_request::NoticeRemoveFieldEnergyByUsingHandCardRequest;
use crate::notify_player_action_info::service::request::notice_search_card_by_using_hand_card_request::NoticeSearchCardByUsingHandCardRequest;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_unit_by_using_hand_card_response::NoticeApplyDamageToEveryUnitByUsingHandCardResponse;

use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_by_using_hand_card_response::NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_unit_by_using_hand_card_response::NoticeApplyDamageToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_attach_energy_to_specific_unit_by_using_hand_card_response::NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_draw_card_by_using_hand_card_response::NoticeDrawCardByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_unit_by_using_hand_card_response::NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_remove_field_energy_by_using_hand_card_response::NoticeRemoveFieldEnergyByUsingHandCardResponse;
use crate::notify_player_action_info::service::response::notice_search_card_by_using_hand_card_response::NoticeSearchCardByUsingHandCardResponse;

#[async_trait]
pub trait NotifyPlayerActionInfoService {
    async fn notice_boost_energy_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_boost_energy_to_specific_unit_by_using_hand_card_request: NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeBoostEnergyToSpecificUnitByUsingHandCardResponse;
    async fn notice_draw_card_by_using_hand_card(
        &mut self,
        notice_draw_card_by_using_hand_card_request: NoticeDrawCardByUsingHandCardRequest)
        -> NoticeDrawCardByUsingHandCardResponse;
    async fn notice_search_card_by_using_hand_card(
        &mut self,
        notice_search_card_by_using_hand_card_request: NoticeSearchCardByUsingHandCardRequest)
        -> NoticeSearchCardByUsingHandCardResponse;
    async fn notice_remove_field_energy_by_using_hand_card(
        &mut self,
        notice_remove_field_energy_by_using_hand_card_request: NoticeRemoveFieldEnergyByUsingHandCardRequest)
        -> NoticeRemoveFieldEnergyByUsingHandCardResponse;
    async fn notice_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self,
        notice_remove_energy_of_specific_unit_by_using_hand_card_request: NoticeRemoveEnergyOfSpecificUnitByUsingHandCardRequest)
        -> NoticeRemoveEnergyOfSpecificUnitByUsingHandCardResponse;
    async fn notice_apply_damage_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_apply_damage_to_specific_unit_by_using_hand_card_request: NoticeApplyDamageToSpecificUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToSpecificUnitByUsingHandCardResponse;
    async fn notice_apply_damage_to_every_unit_by_using_hand_card(
        &mut self,
        notice_apply_damage_to_every_unit_by_using_hand_card_request: NoticeApplyDamageToEveryUnitByUsingHandCardRequest)
        -> NoticeApplyDamageToEveryUnitByUsingHandCardResponse;
    async fn notice_attach_energy_to_specific_unit_by_using_hand_card(
        &mut self,
        notice_attach_energy_to_specific_unit_by_using_hand_card_request: NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest)
        -> NoticeAttachEnergyToSpecificUnitByUsingHandCardResponse;
}