use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use chrono::{Timelike, Utc};
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;

use tokio::sync::Mutex as AsyncMutex;
use tokio::time::timeout;
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::check_connecting::entity::check_connecting_message_form::CheckConnectingMessageForm;

use crate::response_generator::response_type::ResponseType::SEND_MESSAGE_CHECK_CONNECTING;
use crate::check_connecting::repository::check_connecting_repository::CheckConnectingRepository;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::response_generator::response_type::ResponseType;

pub struct CheckConnectingRepositoryImpl{}

impl CheckConnectingRepositoryImpl {
    pub fn new() -> Self { CheckConnectingRepositoryImpl {} }
    pub fn get_instance() -> Arc<AsyncMutex<CheckConnectingRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CheckConnectingRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CheckConnectingRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CheckConnectingRepository for CheckConnectingRepositoryImpl {

    async fn send_message_for_checking_connect (&self, client_socket:ClientSocket) {
        println!("CheckConnectingRepositoryImpl: send_message_for_checking_connect()");

        let receiver_transmitter_channel = client_socket.each_client_receiver_transmitter_channel();
        let address = client_socket.address();
        let send_message_for_checking_connect_response =
            CheckConnectingMessageForm::new(address);

        receiver_transmitter_channel.send(
            Arc::new(
                AsyncMutex::new(
                    SEND_MESSAGE_CHECK_CONNECTING(
                        send_message_for_checking_connect_response)))).await;

        println!("Run_time_out()");
        // receiver_transmitter 생성
        let stream_arc = client_socket.stream();
        let receiver_transmitter_channel_arc = client_socket.each_client_receiver_transmitter_channel();
        let receiver_transmitter_channel_clone = receiver_transmitter_channel_arc.clone();

        tokio::task::spawn(async move {
            println!("Transmitter transmit() loop");


            // 응답을 여부를 확인하며, 정해진 시간 안에(180 초) 응답이 없으면 response 는 Err 값을 갖고 반환
            if let receiver_transmitter_channel = receiver_transmitter_channel_clone {
                loop {
                    // Option<Arc<Mutex<ResponseType>>>
                    let response = timeout(Duration::from_secs(180), receiver_transmitter_channel.receive()).await;

                    // 응답을 확인
                    match response {
                        Ok(response) => {
                            println!("Transmitter receive channel data from Receiver");

                            if let Some(response) = response {
                                let mut client_socket_stream = stream_arc.lock().await;
                                println!("Transmitter lock socket");

                                let response_data = response.lock().await;
                                let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");

                                println!("Transmitting data: {}", json_data);
                                println!("Transmitted time: {}.{} sec", Utc::now().second(), Utc::now().timestamp_subsec_millis());

                                client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");

                                if let ResponseType::CHECK_CONNECTING(check_connecting_response) = &*response_data {
                                    if check_connecting_response.get_is_success() == true {
                                        println!("checked_your_connecting");

                                        // TODO : client 의 상태값에 따른 후속 조치는 여기에 작성합니다. (예를 들어 매칭 포기에 따른 상대방의 무한 대기 방지 조치)
                                        // TODO : 위의 작업을 할 경우, response_type 에 client 상태값을 구조요소에 추가 필요함
                                        // TODO : 또는 is_success 을 삭제하고 작성하여도 됨

                                        break;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // TODO: 타임아웃이 발생한 경우 처리 (예를 들어 메모리를 사용 중인 객체 제거)
                            let client_socket_stream = stream_arc.lock().await;
                            drop(client_socket_stream);

                            println!("Transmitter receive channel data timed out");
                            break;
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }

            println!("Transmitter transmit() loop finished");
        });
    }

    async fn find_account_unique_id_by_address (&self, request_address: &str) -> Option<i32> {
        println!("CheckConnectingRepositoryImpl: find_account_unique_id_by_address()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;

        let connection_context_map_clone = connection_context_repository_guard.connection_context_map().lock().await.clone();
        let connection_context_map_clone_keys = connection_context_map_clone.keys();

        for key_index in connection_context_map_clone_keys {
            let client_socket_index = connection_context_map_clone.get(&key_index).unwrap().lock().await.clone();
            let client_socket_index_address = client_socket_index.address();

            if client_socket_index_address == request_address {
                return Some(key_index.clone());
            }
        }
        None
    }

    async fn find_client_socket_by_account_unique_id (&self, account_id: i32) -> Option<ClientSocket> {
        println!("CheckConnectingRepositoryImpl: find_client_socket_by_account_unique_id()");

        let connection_context_repository_mutex = ConnectionContextRepositoryImpl::get_instance();
        let connection_context_repository_guard = connection_context_repository_mutex.lock().await;

        let connection_context_map_clone = connection_context_repository_guard.connection_context_map().lock().await.clone();
        let find_client_socket_by_account_id = connection_context_map_clone.get(&account_id).unwrap().lock().await.clone();

        return Some(find_client_socket_by_account_id);
    }
}