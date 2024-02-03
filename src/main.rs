use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use crate::battle_match_monitor::service::battle_match_monitor_service::BattleMatchMonitorService;
use crate::battle_match_monitor::service::battle_match_monitor_service_impl::BattleMatchMonitorServiceImpl;

use crate::battle_prepare_task::service::battle_prepare_task_service::BattlePrepareTaskService;
use crate::battle_prepare_task::service::battle_prepare_task_service_impl::BattlePrepareTaskServiceImpl;

use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;

use crate::common::env::env_detector::EnvDetector;

use crate::common::ip_address::local_ip_finder::IPAddress;

use crate::domain_initializer::initializer::DomainInitializer;
use crate::game_battle_field_monitor::controller::game_battle_field_monitor_controller::GameBattleFieldMonitorController;
use crate::game_battle_field_monitor::controller::game_battle_field_monitor_controller_impl::GameBattleFieldMonitorControllerImpl;

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
mod match_waiting_timer;
mod ugly_tests;
mod account_deck_card;
mod shop;
mod account_card;
mod battle_wait_queue;
mod battle_ready_account_hash;
mod battle_match_monitor;
mod game_turn;
mod game_lost_zone;
mod game_deck;
mod game_field_energy;
mod game_tomb;
mod game_field_unit;
mod game_hand;
mod game_main_character;
mod battle_prepare_task;
mod card_library;
mod account_point;
mod card_kinds;
mod card_grade;
mod card_race;
mod deck_configuration_validator;
mod game_battle_field_monitor;

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

    let battle_match_monitor_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let battle_match_monitor_service_mutex = BattleMatchMonitorServiceImpl::get_instance();
            let mut battle_match_monitor_service_guard = battle_match_monitor_service_mutex.lock().await;
            println!("Battle Match Monitor instance found. Executing check_battle_match().");
            let _ = battle_match_monitor_service_guard.check_battle_match().await;
        })
    };

    thread_worker_service_guard.save_async_thread_worker("BattleMatchMonitor", Box::new(battle_match_monitor_function.clone()));
    thread_worker_service_guard.start_thread_worker("BattleMatchMonitor").await;

    let battle_prepare_task_function = || -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {
            let battle_prepare_task_service_mutex = BattlePrepareTaskServiceImpl::get_instance();
            let mut battle_prepare_task_service_guard = battle_prepare_task_service_mutex.lock().await;
            println!("Battle Prepare Task instance found. Executing prepare_for_player_battle().");
            let _ = battle_prepare_task_service_guard.prepare_for_player_battle().await;
        })
    };

    thread_worker_service_guard.save_async_thread_worker("BattlePrepareTask", Box::new(battle_prepare_task_function.clone()));
    thread_worker_service_guard.start_thread_worker("BattlePrepareTask").await;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
