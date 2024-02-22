use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::notify_player_action_info::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::notify_player_action_info::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::notify_player_action_info::entity::player_hand_use_info::PlayerHandUseInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::{Opponent, You};
use crate::notify_player_action_info::entity::used_hand_card_info::UsedHandCardInfo;
use crate::notify_player_action_info::repository::notify_player_action_info_repository::NotifyPlayerActionInfoRepository;
use crate::response_generator::response_type::ResponseType::{NOTIFY_DRAW_COUNT, NOTIFY_DRAWN_CARD_LIST, NOTIFY_HAND_USE};

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

    fn get_player_hand_use_info(&self, used_card_id: i32, used_card_type: KindsEnum) -> PlayerHandUseInfo {

        if used_card_type == KindsEnum::Trap {
            let used_card_info = UsedHandCardInfo::new(-1, used_card_type as i32);
            let mut player_hand_use_map = HashMap::new();
            player_hand_use_map.insert(Opponent, used_card_info);

            return PlayerHandUseInfo::new(player_hand_use_map)
        }

        let used_card_info = UsedHandCardInfo::new(used_card_id, used_card_type as i32);
        let mut player_hand_use_map = HashMap::new();
        player_hand_use_map.insert(Opponent, used_card_info);

        PlayerHandUseInfo::new(player_hand_use_map)
    }

    fn get_player_drawn_card_list_info(&self, drawn_card_list: Vec<i32>) -> PlayerDrawnCardListInfo {
        let mut player_drawn_card_list_map_hash = HashMap::new();
        player_drawn_card_list_map_hash.insert(You, drawn_card_list);

        PlayerDrawnCardListInfo::new(player_drawn_card_list_map_hash)
    }

    fn get_player_draw_count_info(&self, draw_count: i32) -> PlayerDrawCountInfo {
        let mut player_draw_count_map_hash = HashMap::new();
        player_draw_count_map_hash.insert(Opponent, draw_count);

        PlayerDrawCountInfo::new(player_draw_count_map_hash)
    }
}

#[async_trait]
impl NotifyPlayerActionInfoRepository for NotifyPlayerActionInfoRepositoryImpl {
    async fn notify_player_draw_card_with_using_hand_card(
        &mut self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        drawn_card_list: Vec<i32>) -> bool {

        println!("NotifyPlayerActionInfoRepositoryImpl: notify_player_draw_card_with_using_hand_card()");

        let draw_count = drawn_card_list.len() as i32;

        let player_hand_use_info = self.get_player_hand_use_info(used_hand_card_id, used_hand_card_type);
        let player_drawn_card_list_info = self.get_player_drawn_card_list_info(drawn_card_list);
        let player_draw_count_info = self.get_player_draw_count_info(draw_count);

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
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_HAND_USE(player_hand_use_info)))).await;

        // 상대에게는 내가 몇 장을 드로우 했는지 공지
        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DRAW_COUNT(player_draw_count_info)))).await;

        // 스스로에게 드로우한 카드 리스트를 공지
        account_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    NOTIFY_DRAWN_CARD_LIST(player_drawn_card_list_info)))).await;

        true
    }
}