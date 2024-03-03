use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::notify_player_action_info::entity::notify_form_basic_attack_to_unit::NotifyFormBasicAttackToUnit;
use crate::notify_player_action_info::entity::notify_form_use_catastrophic_damage_item_card::NotifyFormUseCatastrophicDamageItemCard;
use crate::notify_player_action_info::entity::notify_form_use_draw_support_card::NotifyFormUseDrawSupportCard;
use crate::notify_player_action_info::entity::notify_form_use_field_energy_increase_item_card::NotifyFormUseFieldEnergyIncreaseItemCard;
use crate::notify_player_action_info::entity::notify_form_use_unit_energy_boost_support_card::NotifyFormUseUnitEnergyBoostSupportCard;
use crate::notify_player_action_info::entity::notify_form_use_field_energy_remove_support_card::NotifyFormUseFieldEnergyRemoveSupportCard;
use crate::notify_player_action_info::entity::notify_form_use_field_energy_to_unit::NotifyFormUseFieldEnergyToUnit;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::notify_player_action_info::entity::notify_form_use_general_energy_card_to_unit::NotifyFormUseGeneralEnergyCardToUnit;
use crate::notify_player_action_info::entity::notify_form_use_instant_unit_death_item_card::NotifyFormUseInstantUnitDeathItemCard;
use crate::notify_player_action_info::entity::notify_form_use_multiple_unit_damage_item_card::NotifyFormUseMultipleUnitDamageItemCard;
use crate::notify_player_action_info::entity::notify_form_use_search_deck_support_card::NotifyFormUseSearchDeckSupportCard;
use crate::notify_player_action_info::entity::notify_form_use_special_energy_card_to_unit::NotifyFormUseSpecialEnergyCardToUnit;
use crate::notify_player_action_info::entity::notify_form_use_unit_energy_remove_item_card::NotifyFormUseUnitEnergyRemoveItemCard;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::response_generator::response_type::ResponseType::*;
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;

pub struct NotifyPlayerActionInfoRepositoryImpl;

impl NotifyPlayerActionInfoRepositoryImpl {
    pub fn new() -> Self {
        NotifyPlayerActionInfoRepositoryImpl { }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionInfoRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionInfoRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionInfoRepository for NotifyPlayerActionInfoRepositoryImpl {
    async fn notice_use_field_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_field_energy_to_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_field_energy_to_unit =
            NotifyFormUseFieldEnergyToUnit::new(
                player_field_energy_map_for_notice,
                player_field_unit_energy_map_for_notice);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_FIELD_ENERGY_TO_UNIT(
                        notify_form_use_field_energy_to_unit)))).await;

        true
    }

    async fn notice_use_general_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_general_energy_to_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();


        let notify_form_use_general_energy_card_to_unit =
            NotifyFormUseGeneralEnergyCardToUnit::new(
                player_hand_use_map_for_notice,
                player_field_unit_energy_map_for_notice);

        // 상대에게 일반 에너지 카드 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_GENERAL_ENERGY_CARD_TO_UNIT(
                        notify_form_use_general_energy_card_to_unit)))).await;

        true
    }

    async fn notice_use_special_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
        player_field_unit_extra_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_special_energy_to_unit()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_special_energy_to_unit =
            NotifyFormUseSpecialEnergyCardToUnit::new(
                player_hand_use_map_for_notice,
                player_field_unit_energy_map_for_notice,
                player_field_unit_extra_effect_map_for_notice);

        // 상대에게 특수 에너지 카드 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_SPECIAL_ENERGY_CARD_TO_UNIT(
                        notify_form_use_special_energy_to_unit)))).await;

        true
    }

    async fn notice_use_unit_energy_boost_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_unit_energy_boost_support()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_unit_energy_boost_support_card =
            NotifyFormUseUnitEnergyBoostSupportCard::new(
                player_hand_use_map_for_notice,
                player_deck_card_use_list_map_for_notice,
                player_field_unit_energy_map_for_notice);

        // 상대에게 에너지 부스트 서포트 카드 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_UNIT_ENERGY_BOOST_SUPPORT_CARD(
                        notify_form_use_unit_energy_boost_support_card)))).await;

        true
    }

    async fn notice_use_draw_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_draw_support()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_draw_support_card =
            NotifyFormUseDrawSupportCard::new(player_hand_use_map_for_notice,
                                              player_draw_count_map_for_notice);

        // 상대에게 드로우 서포트 카드 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_DRAW_SUPPORT_CARD(
                        notify_form_use_draw_support_card)))).await;

        true
    }

    async fn notice_use_search_deck_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_search_count_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_search_deck_support()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_search_deck_support_card =
            NotifyFormUseSearchDeckSupportCard::new(player_hand_use_map_for_notice,
                                                    player_search_count_map_for_notice);

        // 상대에게 덱 검색 서포트 카드 사용 공지 (유닛 검색이 아닌 경우에도 활용 가능)
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_SEARCH_DECK_SUPPORT_CARD(
                        notify_form_use_search_deck_support_card)))).await;

        true
    }

    async fn notice_use_field_energy_remove_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_field_energy_remove_support()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_field_energy_remove_support_card =
            NotifyFormUseFieldEnergyRemoveSupportCard::new(player_hand_use_map_for_notice,
                                                           player_field_energy_map_for_notice);

        // 상대에게 필드 에너지 파괴 서포트 카드 사용 공지 (동시 파괴인 경우에도 활용 가능)
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_FIELD_ENERGY_REMOVE_SUPPORT_CARD(
                        notify_form_use_field_energy_remove_support_card)))).await;

        true
    }

    async fn notice_use_instant_unit_death_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_instant_unit_death_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_instant_unit_death_item_card =
            NotifyFormUseInstantUnitDeathItemCard::new(player_hand_use_map_for_notice,
                                                       player_field_unit_health_point_map_for_notice,
                                                       player_field_unit_death_map_for_notice);

        // 상대 즉사 아이템 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_INSTANT_UNIT_DEATH_ITEM_CARD(
                        notify_form_use_instant_unit_death_item_card)))).await;

        true
    }

    async fn notice_use_field_energy_increase_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_field_energy_increase_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_field_energy_increase_item_card =
            NotifyFormUseFieldEnergyIncreaseItemCard::new(
                player_hand_use_map_for_notice,
                player_field_energy_map_for_notice);

        // 상대 즉사 아이템 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_FIELD_ENERGY_INCREASE_ITEM_CARD(
                        notify_form_use_field_energy_increase_item_card)))).await;

        true
    }

    async fn notice_use_catastrophic_damage_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
        player_deck_card_lost_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_catastrophic_damage_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_catastrophic_damage_item_card =
            NotifyFormUseCatastrophicDamageItemCard::new(
                player_hand_use_map_for_notice,
                player_field_unit_health_point_map_for_notice,
                player_field_unit_death_map_for_notice,
                player_main_character_health_point_map_for_notice,
                player_main_character_survival_map_for_notice,
                player_deck_card_lost_list_map_for_notice);

        // 광역 대미지 아이템 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_CATASTROPHIC_DAMAGE_ITEM_CARD(
                        notify_form_use_catastrophic_damage_item_card)))).await;

        true
    }

    async fn notice_use_unit_energy_remove_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_unit_energy_remove_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_unit_energy_remove_item_card =
            NotifyFormUseUnitEnergyRemoveItemCard::new(
                player_hand_use_map_for_notice,
                player_field_unit_energy_map_for_notice,
                player_field_unit_health_point_map_for_notice,
                player_field_unit_death_map_for_notice);

        // 상대 유닛 에너지 제거 아이템 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_UNIT_ENERGY_REMOVE_ITEM_CARD(
                        notify_form_use_unit_energy_remove_item_card)))).await;

        true
    }

    async fn notice_use_multiple_unit_damage_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_multiple_unit_damage_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_use_multiple_unit_damage_item_card =
            NotifyFormUseMultipleUnitDamageItemCard::new(
                player_hand_use_map_for_notice,
                player_field_unit_health_point_map_for_notice,
                player_field_unit_death_map_for_notice);

        // 다중 대미지 아이템 사용 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_USE_MULTIPLE_UNIT_DAMAGE_ITEM_CARD(
                        notify_form_use_multiple_unit_damage_item_card)))).await;

        true
    }

    async fn notice_basic_attack_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool {


        println!("NotifyPlayerActionInfoRepositoryImpl: notice_use_multiple_unit_damage_item()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let notify_form_basic_attack_to_unit =
            NotifyFormBasicAttackToUnit::new(
                player_field_unit_health_point_map_for_notice,
                player_field_unit_harmful_effect_map_for_notice,
                player_field_unit_death_map_for_notice);

        // 유닛 대상 기본 공격 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_BASIC_ATTACK_TO_UNIT(
                        notify_form_basic_attack_to_unit)))).await;

        true
    }
}