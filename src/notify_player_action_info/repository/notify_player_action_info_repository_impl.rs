use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::notify_player_action_info::entity::attached_energy_info::{AttachedEnergyInfo};
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::notify_player_action_info::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::notify_player_action_info::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::notify_player_action_info::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_hand_card_use_info::PlayerHandCardUseInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::{Opponent, You};
use crate::notify_player_action_info::entity::player_search_card_list_info::PlayerSearchCardListInfo;
use crate::notify_player_action_info::entity::player_search_count_info::PlayerSearchCountInfo;
use crate::notify_player_action_info::entity::used_hand_card_info::UsedHandCardInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::response_generator::response_type::ResponseType::{NOTIFY_DECK_CARD_USE_LIST, NOTIFY_DRAW_COUNT, NOTIFY_DRAWN_CARD_LIST, NOTIFY_FIELD_ENERGY, NOTIFY_FIELD_UNIT_ENERGY, NOTIFY_FIELD_UNIT_HEALTH_POINT, NOTIFY_HAND_CARD_USE, NOTIFY_SEARCH_CARD_LIST, NOTIFY_SEARCH_COUNT};

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

    fn get_player_hand_card_use_info(&self,
                                     notify_player_index: PlayerIndex,
                                     used_card_id: i32,
                                     used_card_type: KindsEnum
    ) -> PlayerHandCardUseInfo {

        if used_card_type == KindsEnum::Trap {
            let used_card_info = UsedHandCardInfo::new(-1, used_card_type as i32);
            let mut player_hand_use_map = HashMap::new();
            player_hand_use_map.insert(notify_player_index, used_card_info);

            return PlayerHandCardUseInfo::new(player_hand_use_map)
        }

        let used_card_info = UsedHandCardInfo::new(used_card_id, used_card_type as i32);
        let mut player_hand_use_map = HashMap::new();
        player_hand_use_map.insert(notify_player_index, used_card_info);

        PlayerHandCardUseInfo::new(player_hand_use_map)
    }

    fn get_player_deck_card_list_use_info(&self,
                                          notify_player_index: PlayerIndex,
                                          used_deck_card_list: Vec<i32>
    ) -> PlayerDeckCardUseListInfo {

        let mut player_deck_card_list_use_map = HashMap::new();
        player_deck_card_list_use_map.insert(notify_player_index, used_deck_card_list);

        PlayerDeckCardUseListInfo::new(player_deck_card_list_use_map)
    }

    // TODO: Aggregate Root 재구성 필요
    fn get_player_field_unit_energy_info(&self,
                                         notify_player_index: PlayerIndex,
                                         unit_index: i32,
                                         attached_energy_info: AttachedEnergyInfo
    ) -> PlayerFieldUnitEnergyInfo {

        let mut field_unit_energy_map = HashMap::new(); // 이 부분
        field_unit_energy_map.insert(unit_index, attached_energy_info);

        let mut player_field_unit_energy_map = HashMap::new();
        player_field_unit_energy_map.insert(notify_player_index, field_unit_energy_map);

        PlayerFieldUnitEnergyInfo::new(player_field_unit_energy_map)
    }

    fn get_player_drawn_card_list_info(&self,
                                       notify_player_index: PlayerIndex,
                                       drawn_card_list: Vec<i32>
    ) -> PlayerDrawnCardListInfo {

        let mut player_drawn_card_list_map = HashMap::new();
        player_drawn_card_list_map.insert(notify_player_index, drawn_card_list);

        PlayerDrawnCardListInfo::new(player_drawn_card_list_map)
    }

    fn get_player_draw_count_info(&self,
                                  notify_player_index: PlayerIndex,
                                  drawn_card_list: Vec<i32>
    ) -> PlayerDrawCountInfo {

        let mut player_draw_count_map = HashMap::new();
        player_draw_count_map.insert(notify_player_index, drawn_card_list.len() as i32);

        PlayerDrawCountInfo::new(player_draw_count_map)
    }

    fn get_player_search_card_list_info(&self,
                                        notify_player_index: PlayerIndex,
                                        found_card_list: Vec<i32>
    ) -> PlayerSearchCardListInfo {

        let mut player_search_card_list_map = HashMap::new();
        player_search_card_list_map.insert(notify_player_index, found_card_list);

        PlayerSearchCardListInfo::new(player_search_card_list_map)
    }

    fn get_player_search_count_info(&self,
                                    notify_player_index: PlayerIndex,
                                    found_card_list: Vec<i32>
    ) -> PlayerSearchCountInfo {

        let mut player_search_count_map = HashMap::new();
        player_search_count_map.insert(notify_player_index, found_card_list.len() as i32);

        PlayerSearchCountInfo::new(player_search_count_map)
    }

    fn get_player_field_energy_info(&self,
                                    notify_player_index: PlayerIndex,
                                    field_energy_count: i32
    ) -> PlayerFieldEnergyInfo {

        let mut player_field_energy_map = HashMap::new();
        player_field_energy_map.insert(notify_player_index, field_energy_count);

        PlayerFieldEnergyInfo::new(player_field_energy_map)
    }

    fn get_player_field_unit_health_point_info(&self,
                                               notify_player_index: PlayerIndex,
                                               field_unit_health_point_info: FieldUnitHealthPointInfo
    ) -> PlayerFieldUnitHealthPointInfo {

        let mut player_field_unit_health_point_info = HashMap::new();
        player_field_unit_health_point_info.insert(notify_player_index, field_unit_health_point_info);

        PlayerFieldUnitHealthPointInfo::new(player_field_unit_health_point_info)
    }
}

#[async_trait]
impl NotifyPlayerActionInfoRepository for NotifyPlayerActionInfoRepositoryImpl {
    async fn notify_player_boost_energy_to_specific_unit_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        found_energy_card_id_list_form_deck: Vec<i32>,
        unit_index: i32,
        attached_energy_info: AttachedEnergyInfo) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_boost_energy_to_specific_unit_by_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        let player_deck_card_list_use_info_for_opponent =
            self.get_player_deck_card_list_use_info(Opponent, found_energy_card_id_list_form_deck.clone());
        let player_deck_card_list_use_info_for_account =
            self.get_player_deck_card_list_use_info(You, found_energy_card_id_list_form_deck.clone());

        // 상대에게 덱에서 추가적으로 사용한 카드 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DECK_CARD_USE_LIST(player_deck_card_list_use_info_for_opponent)))).await;

        // 스스로에게 덱에서 추가적으로 사용한 카드 공지
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DECK_CARD_USE_LIST(player_deck_card_list_use_info_for_account)))).await;

        let player_field_unit_energy_info_for_opponent =
            self.get_player_field_unit_energy_info(Opponent, unit_index, attached_energy_info.clone());
        let player_field_unit_energy_info_for_account =
            self.get_player_field_unit_energy_info(You, unit_index, attached_energy_info.clone());

        // 상대에게 내 필드 유닛의 에너지 정보 업데이트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info_for_opponent)))).await;

        // 스스로에게 내 필드 유닛의 에너지 정보 업데이트 공지
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info_for_account)))).await;

        true
    }

    async fn notify_player_draw_card_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        drawn_card_list: Vec<i32>) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_draw_card_with_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        // 상대에게는 내가 몇 장을 드로우 했는지 공지
        let player_draw_count_info =
            self.get_player_draw_count_info(Opponent, drawn_card_list.clone());

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DRAW_COUNT(player_draw_count_info)))).await;

        // 스스로에게 드로우한 카드 리스트를 공지
        let player_drawn_card_list_info =
            self.get_player_drawn_card_list_info(You, drawn_card_list.clone());

        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DRAWN_CARD_LIST(player_drawn_card_list_info)))).await;

        true
    }

    async fn notify_player_search_card_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        found_card_id_list_from_deck: Vec<i32>) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_search_card_by_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        // 상대에게 몇 장을 검색하여 가져왔는지 공지
        let player_search_count_info_for_opponent =
            self.get_player_search_count_info(Opponent, found_card_id_list_from_deck.clone());

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_SEARCH_COUNT(player_search_count_info_for_opponent)))).await;

        // 스스로에게 검색해 온 카드 리스트 공지
        let player_search_card_list_info_for_account =
            self.get_player_search_card_list_info(You, found_card_id_list_from_deck.clone());

        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_SEARCH_CARD_LIST(player_search_card_list_info_for_account)))).await;

        true
    }

    async fn notify_player_remove_field_energy_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        remaining_field_energy_count: i32) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_remove_field_energy_by_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        let player_field_energy_info_for_opponent =
            self.get_player_field_energy_info(You, remaining_field_energy_count);
        let player_field_energy_info_for_account =
            self.get_player_field_energy_info(Opponent, remaining_field_energy_count);

        // 상대에게 갱신된 필드 에너지 정보 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_ENERGY(player_field_energy_info_for_opponent)))).await;

        // 스스로에게 갱신된 필드 에너지 정보 공지
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_ENERGY(player_field_energy_info_for_account)))).await;

        true
    }

    async fn notify_player_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        opponent_unit_index: i32,
        attached_energy_info: AttachedEnergyInfo) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_remove_energy_of_specific_unit_by_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        let player_field_unit_energy_info_for_opponent =
            self.get_player_field_unit_energy_info(You, opponent_unit_index, attached_energy_info.clone());
        let player_field_unit_energy_info_for_account =
            self.get_player_field_unit_energy_info(Opponent, opponent_unit_index, attached_energy_info.clone());

        // 상대에게 내 필드 유닛의 에너지 정보 업데이트 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info_for_opponent)))).await;

        // 스스로에게 내 필드 유닛의 에너지 정보 업데이트 공지
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_ENERGY(player_field_unit_energy_info_for_account)))).await;

        true
    }

    async fn notify_player_apply_damage_to_specific_unit_by_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        field_unit_health_point_info: FieldUnitHealthPointInfo) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_remove_energy_of_specific_unit_by_using_hand_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let account_socket_option = connection_context_map_guard.get(&account_unique_id);
        let account_socket_mutex = account_socket_option.unwrap();
        let account_socket_guard = account_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();
        let account_receiver_transmitter_channel = account_socket_guard.each_client_receiver_transmitter_channel();

        // 상대에게 무슨 카드를 썼는지 공지
        let player_hand_card_use_info =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_type);

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_CARD_USE(player_hand_card_use_info)))).await;

        let player_field_unit_health_point_info_for_opponent =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());
        let player_field_unit_health_point_info_for_account =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());

        // 상대에게 데미지 적용 후 남은 체력 전송
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_HEALTH_POINT(player_field_unit_health_point_info_for_opponent)))).await;

        // 스스로에게도 데미지 적용 후 남은 체력 전송
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_FIELD_UNIT_HEALTH_POINT(player_field_unit_health_point_info_for_account)))).await;

        true
    }
}