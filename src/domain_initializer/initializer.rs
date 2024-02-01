use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::sync::mpsc;

use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account_card::service::account_card_service_impl::AccountCardServiceImpl;
use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;
use crate::battle_match_monitor::service::battle_match_monitor_service_impl::BattleMatchMonitorServiceImpl;
use crate::battle_prepare_task::service::battle_prepare_task_service_impl::BattlePrepareTaskServiceImpl;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service_impl::BattleReadyAccountHashServiceImpl;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_wait_queue::service::battle_wait_queue_service_impl::BattleWaitQueueServiceImpl;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::card_kinds::service::card_kinds_service::CardKindsService;
use crate::card_kinds::service::card_kinds_service_impl::CardKindsServiceImpl;
use crate::card_library::entity::card_dictionary_label::CardDictionaryLabel;
use crate::card_library::repository::card_library_repository::CardLibraryRepository;
use crate::card_library::repository::card_library_repository_impl::CardLibraryRepositoryImpl;
use crate::card_library::service::card_library_service::CardLibraryService;
use crate::card_library::service::card_library_service_impl::CardLibraryServiceImpl;
use crate::deck_card::service::deck_card_service_impl::DeckCardServiceImpl;

use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::common::csv::csv_reader::{build_dictionaries, csv_read};

use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_worker::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

use crate::common::mpsc::mpsc_creator::mpsc_channel::define_channel;
use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;

use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;

use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;

use crate::response_generator::response_type::ResponseType;

use crate::transmitter::controller::transmitter_controller::TransmitterController;
use crate::transmitter::controller::transmitter_controller_impl::TransmitterControllerImpl;

define_channel!(AcceptorReceiverChannel, ClientSocket);
define_channel!(AcceptorTransmitterChannel, ClientSocket);
define_channel!(ReceiverTransmitterLegacyChannel, Arc<Mutex<ResponseType>>);

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = ServerSocketServiceImpl::get_instance();
    }
    pub fn init_thread_worker_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    pub fn init_account_domain(&self) {
        let _ = AccountServiceImpl::get_instance();
    }
    pub fn init_account_card_domain(&self) { let _ = AccountCardServiceImpl::get_instance(); }
    pub fn init_account_deck_domain(&self) { let _ = AccountDeckServiceImpl::get_instance(); }
    pub fn init_deck_card_domain(&self) { let _ = DeckCardServiceImpl::get_instance(); }

    pub fn init_game_turn_domain(&self) { let _ = GameTurnServiceImpl::get_instance(); }

    pub async fn init_client_socket_accept_domain(&self,
                                                  acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>,
                                                  acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {

        let client_socket_accept_controller_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let mut client_socket_accept_controller = client_socket_accept_controller_mutex.lock().await;

        client_socket_accept_controller.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc).await;
        client_socket_accept_controller.inject_acceptor_transmitter_channel(acceptor_transmitter_channel_arc).await;
        // client_socket_accept_controller.inject_acceptor_transmitter_channel()
    }

    pub async fn init_receiver_domain(&self,
                                      acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>,
                                      receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {
        let server_receiver_controller_mutex = ServerReceiverControllerImpl::get_instance();
        let mut server_receiver_controller = server_receiver_controller_mutex.lock().await;

        server_receiver_controller.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc).await;
        server_receiver_controller.inject_receiver_transmitter_channel(receiver_transmitter_channel_arc).await;
    }

    pub async fn init_transmitter_domain(&self,
                                         acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>,
                                         receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {

        let transmitter_controller_mutex = TransmitterControllerImpl::get_instance();
        let mut transmitter_controller = transmitter_controller_mutex.lock().await;

        transmitter_controller.inject_acceptor_transmitter_channel(acceptor_transmitter_channel_arc).await;
        transmitter_controller.inject_receiver_transmitter_channel(receiver_transmitter_channel_arc).await;
    }

    pub async fn init_battle_wait_queue_domain(&self) {
        let _ = BattleWaitQueueServiceImpl::get_instance();
    }

    pub async fn init_battle_ready_account_hash_domain(&self) {
        let _ = BattleReadyAccountHashServiceImpl::get_instance();
    }

    pub async fn init_battle_match_monitor_domain(&self) {
        let _ = BattleMatchMonitorServiceImpl::get_instance();
    }

    pub async fn init_battle_prepare_task_domain(&self) {
        let _ = BattlePrepareTaskServiceImpl::get_instance();
    }

    pub async fn init_battle_room_domain(&self) {
        let _ = BattleRoomServiceImpl::get_instance();
    }

    pub async fn init_redis_in_memory_domain(&self) {
        let _ = RedisInMemoryServiceImpl::get_instance();
    }

    pub async fn init_card_attribute_domain(&self) {
        // let current_dir = std::env::current_dir().unwrap_or_else(|err| {
        //     eprintln!("Failed to get current directory: {}", err);
        //     std::process::exit(1);
        // });
        // println!("current_dir: {:?}", current_dir);
        //
        // let filename = "../../resources/csv/every_card.csv";
        //
        // let csv_content = csv_read(filename).unwrap_or_else(|err| {
        //     println!("Card 정보 읽는 도중 오류 발생: {}", err);
        //     std::process::exit(1);
        // });
        //
        // let (
        //     race_dictionary,
        //     card_grade_dictionary,
        //     card_kinds_dictionary,
        //     energy_needed_dictionary,
        //     attack_dictionary,
        //     passive_dictionary,
        //     skill_dictionary,
        //     hp_dictionary,
        // ) = build_dictionaries(&csv_content);

        let source = "../../resources/csv/card_data.csv";
        let card_library = CardLibraryServiceImpl::get_instance();
        let card_library_guard = card_library.lock().await;

        card_library_guard.open_library(source).await;

        let kind_dictionary = card_library_guard.get_dictionary(CardDictionaryLabel::Kind).await;

        drop(card_library_guard);

        CardKindsRepositoryImpl::create_instance(kind_dictionary).await;
        let card_kinds_repository = CardKindsRepositoryImpl::get_instance();
        let card_kinds_repository_guard = card_kinds_repository.lock().await;

        let result = card_kinds_repository_guard.get_card_kind("6").await;
        println!("card kinds: {:?}", result);

        drop(card_kinds_repository_guard);

        let card_kinds_service = CardKindsServiceImpl::get_instance();
        let card_kinds_service_guard = card_kinds_service.lock().await;

        let result_from_service = card_kinds_service_guard.get_card_kind("6").await;
        println!("card kinds from service: {:?}", result_from_service);

        if result_from_service.map(|kind| kind == "1").unwrap_or(false) {
            println!("휴먼 카드 감지!");
        }

        drop(card_kinds_service_guard);
    }

    // TODO: library 사용 방식이 확정되면 추가할 것
    // pub async fn init_card_library_domain(&self) {
    //     let source = "../../resources/csv/every_card.csv";
    //     let card_library = CardLibraryServiceImpl::get_instance();
    //     let card_library_guard = card_library.lock().await;
    //
    //     card_library_guard.open_library(source).await;
    //
    //     //
    //     //
    //     // let kind_dictionary = card_library_repo_guard.get_dictionary(CardDictionaryLabel::Kind).await;
    //     // CardKindsRepositoryImpl::create_instance(kind_dictionary).await;
    //     //
    //     // let card_kinds_repository = CardKindsRepositoryImpl::get_instance();
    //     // let card_kinds_repository_guard = card_kinds_repository.lock().await;
    //     //
    //     // let result = card_kinds_repository_guard.get_card_kind("6").await;
    //     // println!("card kinds: {:?}", result);
    //     //
    //     // drop(card_kinds_repository_guard);
    // }

    pub async fn init_every_domain(&self) {
        /* IPC Channel List */
        let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
        let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());

        let acceptor_transmitter_channel = AcceptorTransmitterChannel::new(1);
        let acceptor_transmitter_channel_arc = Arc::new(acceptor_transmitter_channel.clone());

        let receiver_transmitter_channel = ReceiverTransmitterLegacyChannel::new(1);
        let receiver_transmitter_channel_arc = Arc::new(receiver_transmitter_channel.clone());

        /* Business Domain List */
        self.init_account_domain();
        self.init_account_card_domain();
        self.init_account_deck_domain();
        self.init_deck_card_domain();

        /* Core Domain List */
        self.init_server_socket_domain();
        self.init_thread_worker_domain();
        self.init_client_socket_accept_domain(
            acceptor_receiver_channel_arc.clone(), acceptor_transmitter_channel_arc.clone()).await;
        self.init_receiver_domain(
            acceptor_receiver_channel_arc.clone(), receiver_transmitter_channel_arc.clone()).await;
        self.init_transmitter_domain(
            acceptor_transmitter_channel_arc.clone(), receiver_transmitter_channel_arc.clone()).await;

        /* Battle Matching Domain List */
        self.init_battle_wait_queue_domain().await;
        self.init_battle_ready_account_hash_domain().await;
        self.init_battle_room_domain().await;
        self.init_battle_match_monitor_domain().await;
        self.init_battle_prepare_task_domain().await;

        /* In-game Object Domain List */
        self.init_game_turn_domain();

        /* Redis In-Memory DB Domain */
        self.init_redis_in_memory_domain().await;

        /* Card Attribute Domain */
        self.init_card_attribute_domain().await;
        // self.init_card_library_domain().await;
    }
}

