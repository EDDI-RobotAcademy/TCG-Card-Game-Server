use async_trait::async_trait;
use crate::notify_player_action_info::service::request::notice_apply_damage_to_every_opponent_unit_request::{NoticeApplyDamageToEveryOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_opponent_main_character_request::NoticeApplyDamageToOpponentMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_boost_energy_to_specific_unit_request::{NoticeBoostEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_apply_damage_to_specific_opponent_unit_request::{NoticeApplyDamageToSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_attach_energy_to_specific_unit_request::{NoticeAttachEnergyToSpecificUnitRequest};
use crate::notify_player_action_info::service::request::notice_draw_card_request::{NoticeDrawCardRequest};
use crate::notify_player_action_info::service::request::notice_instant_death_of_specific_opponent_unit_request::{NoticeInstantDeathOfSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_lost_deck_card_of_opponent_request::{NoticeLostDeckCardOfOpponentRequest};
use crate::notify_player_action_info::service::request::notice_remove_energy_of_specific_opponent_unit_request::{NoticeRemoveEnergyOfSpecificOpponentUnitRequest};
use crate::notify_player_action_info::service::request::notice_remove_field_energy_of_opponent_request::{NoticeRemoveFieldEnergyOfOpponentRequest};
use crate::notify_player_action_info::service::request::notice_search_card_request::{NoticeSearchCardRequest};
use crate::notify_player_action_info::service::request::notice_use_hand_card_request::NoticeUseHandCardRequest;
use crate::notify_player_action_info::service::response::notice_apply_damage_to_every_opponent_unit_response::{NoticeApplyDamageToEveryOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_opponent_main_character_response::NoticeApplyDamageToOpponentMainCharacterResponse;

use crate::notify_player_action_info::service::response::notice_boost_energy_to_specific_unit_response::{NoticeBoostEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_apply_damage_to_specific_opponent_unit_response::{NoticeApplyDamageToSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_attach_energy_to_specific_unit_response::{NoticeAttachEnergyToSpecificUnitResponse};
use crate::notify_player_action_info::service::response::notice_draw_card_response::{NoticeDrawCardResponse};
use crate::notify_player_action_info::service::response::notice_instant_death_of_specific_opponent_unit_response::{NoticeInstantDeathOfSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_lost_deck_card_opponent_response::{NoticeLostDeckCardOfOpponentResponse};
use crate::notify_player_action_info::service::response::notice_remove_energy_of_specific_opponent_unit_response::{NoticeRemoveEnergyOfSpecificOpponentUnitResponse};
use crate::notify_player_action_info::service::response::notice_remove_field_energy_of_opponent_response::{NoticeRemoveFieldEnergyOfOpponentResponse};
use crate::notify_player_action_info::service::response::notice_search_card_response::{NoticeSearchCardResponse};
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[async_trait]
pub trait NotifyPlayerActionInfoService {
    async fn notice_use_hand_card(
        &mut self, notice_use_hand_card_request: NoticeUseHandCardRequest)
        -> NoticeUseHandCardResponse;
    async fn notice_boost_energy_to_specific_unit(
        &mut self,
        notice_boost_energy_to_specific_unit_request: NoticeBoostEnergyToSpecificUnitRequest)
        -> NoticeBoostEnergyToSpecificUnitResponse;
    async fn notice_draw_card(
        &mut self,
        notice_draw_card_request: NoticeDrawCardRequest)
        -> NoticeDrawCardResponse;
    async fn notice_search_card(
        &mut self,
        notice_search_card_request: NoticeSearchCardRequest)
        -> NoticeSearchCardResponse;
    async fn notice_remove_field_energy_of_opponent(
        &mut self,
        notice_remove_field_energy_of_opponent_request: NoticeRemoveFieldEnergyOfOpponentRequest)
        -> NoticeRemoveFieldEnergyOfOpponentResponse;
    async fn notice_remove_energy_of_specific_opponent_unit(
        &mut self,
        notice_remove_energy_of_specific_opponent_unit_request: NoticeRemoveEnergyOfSpecificOpponentUnitRequest)
        -> NoticeRemoveEnergyOfSpecificOpponentUnitResponse;
    async fn notice_apply_damage_to_specific_opponent_unit(
        &mut self,
        notice_apply_damage_to_specific_opponent_unit_request: NoticeApplyDamageToSpecificOpponentUnitRequest)
        -> NoticeApplyDamageToSpecificOpponentUnitResponse;
    async fn notice_apply_damage_to_every_opponent_unit(
        &mut self,
        notice_apply_damage_to_every_opponent_unit_request: NoticeApplyDamageToEveryOpponentUnitRequest)
        -> NoticeApplyDamageToEveryOpponentUnitResponse;
    async fn notice_apply_damage_to_opponent_main_character(
        &mut self,
        notice_apply_damage_to_opponent_main_character_request: NoticeApplyDamageToOpponentMainCharacterRequest)
        -> NoticeApplyDamageToOpponentMainCharacterResponse;
    async fn notice_lost_deck_card_of_opponent(
        &mut self,
        notice_lost_deck_card_of_opponent_request: NoticeLostDeckCardOfOpponentRequest)
        -> NoticeLostDeckCardOfOpponentResponse;
    async fn notice_attach_energy_to_specific_unit(
        &mut self,
        notice_attach_energy_to_specific_unit_request: NoticeAttachEnergyToSpecificUnitRequest)
        -> NoticeAttachEnergyToSpecificUnitResponse;
    async fn notice_instant_death_of_specific_opponent_unit(
        &mut self,
        notice_instant_death_of_specific_opponent_unit_request: NoticeInstantDeathOfSpecificOpponentUnitRequest)
        -> NoticeInstantDeathOfSpecificOpponentUnitResponse;
}