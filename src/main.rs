use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
// use crate::battle_ready_monitor::controller::battle_ready_monitor_controller::BattleReadyMonitorController;
use crate::battle_ready_monitor::controller::battle_ready_monitor_controller_impl::BattleReadyMonitorControllerImpl;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::common::env::env_detector::EnvDetector;
use crate::common::ip_address::local_ip_finder::IPAddress;
use crate::domain_initializer::initializer::DomainInitializer;
use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;
use crate::server_socket::service::server_socket_service::ServerSocketService;
use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_worker::service::thread_worker_service::ThreadWorkerServiceTrait;
use crate::thread_worker::service::thread_worker_service_impl::ThreadWorkerServiceImpl;
use crate::transmitter::controller::transmitter_controller::TransmitterController;
use crate::transmitter::controller::transmitter_controller_impl::TransmitterControllerImpl;

mod thread_worker;
mod common;
mod domain_initializer;
mod server_socket;
mod client_socket_accept;
mod receiver;
mod request_generator;
mod account;
mod mysql_config;
mod transmitter;
mod response_generator;
mod redis;
mod client_program;
mod battle_room;
mod account_deck;
mod battle_ready_monitor;
mod match_waiting_timer;
mod ugly_tests;
mod deck_card;
mod shop;
mod account_card;
mod battle_wait_queue;

#[tokio::main]
async fn main() {
    let domain_initializer = DomainInitializer;
    domain_initializer.init_every_domain().await;

    let server_socket_service = ServerSocketServiceImpl::get_instance();

    let ip = IPAddress::get_local_ip_from_google().unwrap();
    let port = EnvDetector::get_port().unwrap();

    let binding = format!("{}:{}", ip, port).to_string();
    let address = binding.as_str();;

    let mut server_socket_service_guard = server_socket_service.lock().await;

    match server_socket_service_guard.server_socket_bind(address).await {
        Ok(()) => {
            println!("Server bound to address: {}", address);
        }
        Err(err) => {
            eprintln!("Error binding socket: {:?}", err);
        }
    }

    drop(server_socket_service_guard);

    let acceptor_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let client_socket_accept_controller = ClientSocketAcceptControllerImpl::get_instance();
            let client_socket_accept_controller_guard = client_socket_accept_controller.lock().await;
            println!("Controller instance found. Executing accept_client().");
            client_socket_accept_controller_guard.accept_client().await;
        })
    };

    let thread_worker_service = ThreadWorkerServiceImpl::get_instance();
    let mut thread_worker_service_guard = thread_worker_service.lock().unwrap();

    thread_worker_service_guard.save_async_thread_worker("Acceptor", Box::new(acceptor_function.clone()));
    thread_worker_service_guard.start_thread_worker("Acceptor").await;

    let receiver_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let server_receiver_controller_mutex = ServerReceiverControllerImpl::get_instance();
            let mut receiver_guard = server_receiver_controller_mutex.lock().await;
            println!("Receiver instance found. Executing client_receive().");
            let _ = receiver_guard.client_receive().await;
        })
    };

    thread_worker_service_guard.save_async_thread_worker("Receiver", Box::new(receiver_function.clone()));
    thread_worker_service_guard.start_thread_worker("Receiver").await;

    let receiver_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let transmitter_controller_mutex = TransmitterControllerImpl::get_instance();
            let mut transmitter_guard = transmitter_controller_mutex.lock().await;
            println!("Transmitter instance found. Executing transmit_to_client().");
            let _ = transmitter_guard.transmit_to_client().await;
        })
    };

    thread_worker_service_guard.save_async_thread_worker("Transmitter", Box::new(receiver_function.clone()));
    thread_worker_service_guard.start_thread_worker("Transmitter").await;

    // TODO: 현재 상황에서 굳이 필요한가 ? (나중에 성능 문제로 필요하면 생각하자)
    // let battle_ready_monitor_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
    //     Box::pin(async {
    //         let battle_ready_monitor_controller_mutex = BattleReadyMonitorControllerImpl::get_instance();
    //         let mut battle_ready_monitor_guard = battle_ready_monitor_controller_mutex.lock().await;
    //         println!("Battle Ready Monitor instance found. Executing transmit_to_client().");
    //         let _ = battle_ready_monitor_guard.start_monitor_for_battle_match().await;
    //     })
    // };
    //
    // thread_worker_service_guard.save_async_thread_worker("BattleReadyMonitor", Box::new(battle_ready_monitor_function.clone()));
    // thread_worker_service_guard.start_thread_worker("BattleReadyMonitor").await;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
