use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::notify_player_action::repository::notify_player_action_repository::NotifyPlayerActionRepository;
use crate::notify_player_action::service::response::notify_opponent_hand_to_unit_action_response::NotifyOpponentHandToFieldUnitActionResponse;
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
                    ResponseType::NOTIFY_OPPONENT_HAND_TO_UNIT_ACTION(
                        NotifyOpponentHandToFieldUnitActionResponse::new(unit_card_number))))).await;

        true
    }
}