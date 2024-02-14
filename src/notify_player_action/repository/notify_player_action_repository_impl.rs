use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::notify_player_action::entity::notify_opponent_increase_field_energy_item_usage::NotifyOpponentIncreaseFieldEnergyItemUsage;
use crate::notify_player_action::entity::notify_opponent_remove_field_energy_support_usage::NotifyOpponentRemoveFieldEnergySupportUsage;
use crate::notify_player_action::entity::notify_opponent_search_support_usage::NotifyOpponentSearchSupportUsage;
use crate::notify_player_action::entity::notify_opponent_to_draw_support_usage::NotifyOpponentToDrawSupportUsage;
use crate::notify_player_action::entity::notify_opponent_to_energy_boost::NotifyOpponentToEnergyBoost;
use crate::notify_player_action::entity::notify_opponent_to_energy_usage::NotifyOpponentToEnergyUsage;
use crate::notify_player_action::entity::notify_opponent_to_enhance_attack_point_tool_usage::NotifyOpponentToEnhanceAttackPointToolUsage;
use crate::notify_player_action::entity::notify_opponent_to_instant_death_item_alternatives_usage::NotifyOpponentToInstantDeathItemAlternativesUsage;
use crate::notify_player_action::entity::notify_opponent_to_instant_death_item_usage::NotifyOpponentToInstantDeathItemUsage;
use crate::notify_player_action::entity::notify_opponent_to_unit_deploy::NotifyOpponentToUnitDeploy;
use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::response_generator::response_type::ResponseType;

// TODO: 추후 HashMap을 구성하여 한 번 등록된 사용자에 대한 처리는 즉시 진행되도록 구성해
pub struct NotifyPlayerActionRepositoryImpl;

impl NotifyPlayerActionRepositoryImpl {
    pub fn new() -> Self {
        NotifyPlayerActionRepositoryImpl { }
    }

    pub fn get_instance() -> Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<NotifyPlayerActionRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        NotifyPlayerActionRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl NotifyPlayerActionRepository for NotifyPlayerActionRepositoryImpl {
    async fn notify_to_opponent_what_you_do(&mut self, opponent_unique_id: i32, unit_card_number: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_what_you_do()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;
        // let opponent_socket = opponent_socket_guard.stream();
        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_UNIT_DEPLOY(
                        NotifyOpponentToUnitDeploy::new(unit_card_number))))).await;

        true
    }

    async fn notify_to_opponent_you_use_energy_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_energy_card_id: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_what_you_do()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;
        // let opponent_socket = opponent_socket_guard.stream();
        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_ENERGY_USAGE(
                        NotifyOpponentToEnergyUsage::new(unit_card_index, usage_energy_card_id))))).await;

        true
    }

    async fn notify_to_opponent_you_use_energy_boost_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_support_card_id: i32, boosting_energy_count: i32, boosting_energy_card_id: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_you_use_energy_boost_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        let mut boosting_energy_card_id_list = Vec::new();

        for _ in 0..boosting_energy_count {
            boosting_energy_card_id_list.push(boosting_energy_card_id);
        }

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_ENERGY_BOOST(
                        NotifyOpponentToEnergyBoost::new(usage_support_card_id, unit_card_index, boosting_energy_card_id_list))))).await;

        true
    }

    // TODO: 에너지 사용 여부도 필요함 (필드, 핸드)
    async fn notify_to_opponent_you_use_item_instant_death_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_you_use_item_instant_death_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_INSTANT_DEATH_ITEM_USAGE(
                        NotifyOpponentToInstantDeathItemUsage::new(unit_card_index, usage_item_card_id))))).await;

        true
    }

    async fn notify_to_opponent_you_use_item_instant_death_card_alternatives(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32, alternatives_damage: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_you_use_item_instant_death_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_INSTANT_DEATH_ITEM_ALTERNATIVES_USAGE(
                        NotifyOpponentToInstantDeathItemAlternativesUsage::new(unit_card_index, usage_item_card_id, alternatives_damage))))).await;

        true
    }

    async fn notify_to_opponent_you_use_draw_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, draw_card_count: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_you_use_draw_support_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_TO_DRAW_SUPPORT_USAGE(
                        NotifyOpponentToDrawSupportUsage::new(usage_support_card_id, draw_card_count))))).await;

        true
    }

    async fn notify_opponent_you_use_search_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, found_card_count: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_opponent_you_use_search_support_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_SEARCH_SUPPORT_USAGE(
                        NotifyOpponentSearchSupportUsage::new(usage_support_card_id, found_card_count))))).await;

        true
    }

    async fn notify_opponent_you_use_remove_field_energy_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, amount_to_remove: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_opponent_you_use_search_support_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_REMOVE_FIELD_ENERGY_SUPPORT_USAGE(
                        NotifyOpponentRemoveFieldEnergySupportUsage::new(usage_support_card_id, amount_to_remove))))).await;

        true
    }

    async fn notify_opponent_you_use_item_field_energy_increase_card(&mut self, opponent_unique_id: i32, usage_item_card_id: i32, increased_field_energy: i32) -> bool {
        println!("NotifyPlayerActionRepositoryImpl: notify_opponent_you_use_item_field_energy_increase_card()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
        let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
        let connection_context_map_guard = connection_context_map_mutex.lock().await;

        let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
        let opponent_socket_mutex = opponent_socket_option.unwrap();
        let opponent_socket_guard = opponent_socket_mutex.lock().await;

        let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

        opponent_receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    ResponseType::NOTIFY_OPPONENT_INCREASE_FIELD_ENERGY_ITEM_USAGE(NotifyOpponentIncreaseFieldEnergyItemUsage::new(usage_item_card_id, increased_field_energy))))).await;

        true
    }

async fn notify_to_opponent_you_use_tool_card_to_enhance_attack_point(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32) -> bool {
    println!("NotifyPlayerActionRepositoryImpl: notify_to_opponent_you_use_tool_card_to_enhance_attack_point()");

    let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
    let connection_context_repository_guard = connection_context_repository_mutex.lock().await;
    let connection_context_map_mutex = connection_context_repository_guard.connection_context_map();
    let connection_context_map_guard = connection_context_map_mutex.lock().await;

    let opponent_socket_option = connection_context_map_guard.get(&opponent_unique_id);
    let opponent_socket_mutex = opponent_socket_option.unwrap();
    let opponent_socket_guard = opponent_socket_mutex.lock().await;

    let opponent_receiver_transmitter_channel = opponent_socket_guard.each_client_receiver_transmitter_channel();

    opponent_receiver_transmitter_channel.send(
        Arc::new(
            AsyncMutex::new(
                ResponseType::NOTIFY_OPPONENT_TO_ENHANCE_ATTACK_POINT_TOOL_USAGE(
                    NotifyOpponentToEnhanceAttackPointToolUsage::new(unit_card_index, usage_item_card_id))))).await;
        true
    }

}
