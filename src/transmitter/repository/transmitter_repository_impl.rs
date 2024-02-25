use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use tokio::time::timeout;
use crate::connection_context::repository::connection_context_repository::ConnectionContextRepository;
use crate::connection_context::repository::connection_context_repository_impl::ConnectionContextRepositoryImpl;
use crate::domain_initializer::initializer::{AcceptorTransmitterChannel, ReceiverTransmitterLegacyChannel};
use crate::match_waiting_timer::repository::match_waiting_timer_repository_impl::MatchWaitingTimerRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;
use crate::response_generator::response_type::ResponseType;

use crate::transmitter::entity::transmit_data::TransmitData;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

pub struct TransmitterRepositoryImpl {
    transmit_data: TransmitData,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>,
    receiver_transmitter_channel_arc: Option<Arc<ReceiverTransmitterLegacyChannel>>,
    connection_context_repository: Arc<AsyncMutex<ConnectionContextRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
}

impl TransmitterRepositoryImpl {
    pub fn new(connection_context_repository: Arc<AsyncMutex<ConnectionContextRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,) -> Self {
        TransmitterRepositoryImpl {
            transmit_data: TransmitData::new(),
            acceptor_transmitter_channel_arc: None,
            receiver_transmitter_channel_arc: None,
            connection_context_repository,
            redis_in_memory_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<TransmitterRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<TransmitterRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        TransmitterRepositoryImpl::new(
                            ConnectionContextRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl TransmitterRepository for TransmitterRepositoryImpl {
    // TODO: Dirty <- Need to Refactor!
    async fn transmit(&mut self) {
        println!("TransmitterRepositoryImpl: transmit()");

        let acceptor_channel = self.acceptor_transmitter_channel_arc.clone();
        let receiver_channel = self.receiver_transmitter_channel_arc.clone();

        let redis_in_memory_repository = self.redis_in_memory_repository.clone();
        let connection_context_repository = self.connection_context_repository.clone();

        // Arc<Mutex<TcpStream>>
        // while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
        //     // Option<Arc<ReceiverTransmitterLegacyChannel>>
        //     let receiver_channel_clone = receiver_channel.clone();
        //
        //     // 변경된 부분: tokio::task::spawn을 사용하여 각 클라이언트에 대한 통신을 별도의 태스크로 분리
        //     tokio::task::spawn(async move {
        //         println!("Transmitter transmit() loop");
        //
        //         // Arc<ReceiverTransmitterLegacyChannel>
        //         if let Some(receiver_transmitter_channel) = receiver_channel_clone {
        //             loop {
        //                 // 변경된 부분: 각 클라이언트에 대한 비동기 통신 작업을 별도의 태스크로 생성하여 동시에 수행
        //                 // Option<Arc<Mutex<ResponseType>>>
        //                 let response = receiver_transmitter_channel.receive().await;
        //                 println!("Transmitter receive channel data from Receiver");
        //
        //                 // Arc<Mutex<ResponseType>>
        //                 if let Some(response) = response {
        //                     // MutexGuard<TcpStream>
        //                     let mut client_socket_stream = stream_arc.lock().await;
        //                     println!("Transmitter lock socket");
        //
        //                     // MutexGaurd<ResponseType>
        //                     let response_data = response.lock().await;
        //                     let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");
        //
        //                     println!("Transmitting data: {}", json_data);
        //
        //                     client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");
        //                 }
        //
        //                 tokio::time::sleep(Duration::from_millis(500)).await;
        //             }
        //         }
        //
        //         println!("Transmitter transmit() loop finished");
        //     });
        // }

        while let Some(client_socket) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
            // ClientSocket {
            //     address: String,
            //     stream: Arc<Mutex<TcpStream>>,
            //     each_client_receiver_transmitter_channel: Arc<ReceiverTransmitterChannel>,
            // }
            let stream_arc = client_socket.stream();
            let receiver_transmitter_channel_arc = client_socket.each_client_receiver_transmitter_channel();
            let receiver_transmitter_channel_clone = receiver_transmitter_channel_arc.clone();

            let redis_in_memory_repository_clone = redis_in_memory_repository.clone();
            let connection_context_repository_clone = connection_context_repository.clone();

            // 변경된 부분: tokio::task::spawn을 사용하여 각 클라이언트에 대한 통신을 별도의 태스크로 분리
            tokio::task::spawn(async move {
                println!("Transmitter transmit() loop");

                // Arc<ReceiverTransmitterLegacyChannel>
                if let receiver_transmitter_channel = receiver_transmitter_channel_clone {
                    loop {
                        // 변경된 부분: 각 클라이언트에 대한 비동기 통신 작업을 별도의 태스크로 생성하여 동시에 수행
                        // Option<Arc<Mutex<ResponseType>>>
                        let response = timeout(Duration::from_secs(1), receiver_transmitter_channel.receive()).await;

                        // 결과를 확인
                        match response {
                            Ok(response) => {
                                println!("Transmitter receive channel data from Receiver");

                                // Arc<Mutex<ResponseType>>
                                if let Some(response) = response {
                                    // MutexGuard<TcpStream>
                                    let mut client_socket_stream = stream_arc.lock().await;
                                    println!("Transmitter lock socket");

                                    // MutexGaurd<ResponseType>
                                    let response_data = response.lock().await;
                                    let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");

                                    println!("Transmitting data: {}", json_data);

                                    client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");

                                    // TODO: Dirty <- Need to Refactor!
                                    if let ResponseType::ACCOUNT_LOGIN(login_response) = &*response_data {
                                        if login_response.get_redis_token() != "" {
                                            println!("로그인 성공: Connection Context 생성");

                                            let mut redis_in_memory_repository_guard = redis_in_memory_repository_clone.lock().await;
                                            let account_unique_id_option_string = redis_in_memory_repository_guard.get(login_response.get_redis_token()).await;
                                            let account_unique_id_string = account_unique_id_option_string.unwrap();
                                            let account_unique_id: i32 = account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

                                            let mut connection_context_repository = connection_context_repository_clone.lock().await;
                                            // connection_context_repository.add_connection_context(account_unique_id,
                                            //                                                      client_socket.stream(),
                                            //                                                      client_socket.each_client_receiver_transmitter_channel()).await;
                                            connection_context_repository.add_connection_context(account_unique_id,
                                                                                                 Arc::new(AsyncMutex::new(client_socket.clone()))).await;
                                        }
                                    }

                                    if let ResponseType::FAKE_BATTLE_ROOM_CREATION(create_fake_battle_room_response_form) = &*response_data {
                                        let fake_your_session = create_fake_battle_room_response_form.get_first_fake_session();
                                        let fake_opponent_session = create_fake_battle_room_response_form.get_second_fake_session();

                                        if fake_your_session != "" && fake_opponent_session != "" {
                                            println!("Fake Battle Room 생성 성공: Fake Connection Context 생성");

                                            let mut redis_in_memory_repository_guard = redis_in_memory_repository_clone.lock().await;
                                            let fake_your_unique_id_option_string = redis_in_memory_repository_guard.get(fake_your_session).await;
                                            let fake_your_unique_id_string = fake_your_unique_id_option_string.unwrap();
                                            let fake_your_unique_id: i32 = fake_your_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");

                                            let fake_opponent_unique_id_option_string = redis_in_memory_repository_guard.get(fake_opponent_session).await;
                                            let fake_opponent_unique_id_string = fake_opponent_unique_id_option_string.unwrap();
                                            let fake_opponent_unique_id: i32 = fake_opponent_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32");
                                            drop(redis_in_memory_repository_guard);

                                            let mut connection_context_repository = connection_context_repository_clone.lock().await;
                                            // connection_context_repository.add_connection_context(account_unique_id,
                                            //                                                      client_socket.stream(),
                                            //                                                      client_socket.each_client_receiver_transmitter_channel()).await;
                                            connection_context_repository.add_connection_context(fake_your_unique_id,
                                                                                                 Arc::new(AsyncMutex::new(client_socket.clone()))).await;

                                            connection_context_repository.add_connection_context(fake_opponent_unique_id,
                                                                                                 Arc::new(AsyncMutex::new(client_socket.clone()))).await;
                                            drop(connection_context_repository);
                                        }
                                    }

                                    if let ResponseType::PROGRAM_EXIT(exit_response) = &*response_data {
                                        if exit_response.does_client_exit_success() {
                                            println!("종료 요청 수신: 전용 Transmitter 종료");
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                // 타임아웃이 발생한 경우 처리
                                // TODO: 당장 여기서 처리할 일이 아직은 없음
                                // println!("Transmitter receive channel data timed out");
                            }
                        }

                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }

                println!("Transmitter transmit() loop finished");
            });
        }

        println!("TransmitterRepositoryImpl: transmit() end");
    }

    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterRepository: inject_accept_transmitter_channel()");

        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {
        println!("TransmitterRepository: inject_receiver_transmitter_channel()");

        self.receiver_transmitter_channel_arc = Option::from(receiver_transmitter_channel_arc);
    }
}

