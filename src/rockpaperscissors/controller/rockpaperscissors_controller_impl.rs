use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;

use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rockpaperscissors::controller::request_form::check_winner_request_form::CheckWinnerRequestForm;
use crate::rockpaperscissors::controller::request_form::rockpaperscissors_request_form::RockpaperscissorsRequestForm;
use crate::rockpaperscissors::controller::response_form::check_winner_response_form::CheckWinnerResponseForm;
use crate::rockpaperscissors::controller::response_form::rockpaperscissors_response_form::RockpaperscissorsResponseForm;
use crate::rockpaperscissors::controller::rockpaperscissors_controller::RockpaperscissorsController;
use crate::rockpaperscissors::service::request::check_winner_request::CheckWinnerRequest;
use crate::rockpaperscissors::service::request::wait_queue_request::WaitQueueRequest;
use crate::rockpaperscissors::service::rockpaperscissors_service::RockpaperscissorsService;
use crate::rockpaperscissors::service::rockpaperscissors_service_impl::RockpaperscissorsServiceImpl;

pub struct RockpaperscissorsControllerImpl {
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    rockpaperscissors_service: Arc<AsyncMutex<RockpaperscissorsServiceImpl>>,

}

impl RockpaperscissorsControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               rockpaperscissors_service: Arc<AsyncMutex<RockpaperscissorsServiceImpl>>,) -> Self {

        RockpaperscissorsControllerImpl {
            battle_room_service,
            game_protocol_validation_service,
            redis_in_memory_service,
            rockpaperscissors_service,


        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<RockpaperscissorsControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockpaperscissorsControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RockpaperscissorsControllerImpl::new(
                            BattleRoomServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            RockpaperscissorsServiceImpl::get_instance(),
                           )));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_valid_protocol(&self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(check_protocol_hacking_request).await;
        drop(game_protocol_validation_service_guard);
        check_protocol_hacking_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }
    async fn get_opponent_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> i32 {
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(find_opponent_by_account_id_request).await;
        drop(battle_room_service_guard);
        find_opponent_by_account_id_response.get_opponent_unique_id()
    }
}

#[async_trait]
impl RockpaperscissorsController for RockpaperscissorsControllerImpl {
    async fn execute_rockpaperscissors_procedure(&self, rockpaperscissors_request_form: RockpaperscissorsRequestForm) -> RockpaperscissorsResponseForm {
        println!("RockpaperscissorsControllerImpl: execute_rockpaperscissors_procedure()");


        let account_unique_id = self.is_valid_session(rockpaperscissors_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return RockpaperscissorsResponseForm::new( false)
        }
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                rockpaperscissors_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();


        let mut rockpaperscissors_service_guard = self.rockpaperscissors_service.lock().await;
        let wait_queue_response=rockpaperscissors_service_guard.enqueue_player_tuple_to_wait_queue(
                                                WaitQueueRequest::new(rockpaperscissors_request_form.get_session_id().to_string(),
                                                                      rockpaperscissors_request_form.get_choice().to_string())).await;
        if wait_queue_response.get_is_success() == false {
            return RockpaperscissorsResponseForm::new(false)
        }
        return RockpaperscissorsResponseForm::new(true);

    }

    async fn execute_check_winner_procedure(&self, check_winner_request_form: CheckWinnerRequestForm) -> CheckWinnerResponseForm {
        println!("RockpaperscissorsControllerImpl: execute_rockpaperscissors_procedure()");
        let account_unique_id = self.is_valid_session(check_winner_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return CheckWinnerResponseForm::new( true,true)
        }
        let mut battle_room_service_guard = self.battle_room_service.lock().await;
        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                check_winner_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await.get_opponent_unique_id();
        let mut rockpaperscissors_service_guard = self.rockpaperscissors_service.lock().await;
        let winner_response=rockpaperscissors_service_guard.check_winner(CheckWinnerRequest::new(account_unique_id)).await;

        return CheckWinnerResponseForm::new( winner_response.get_am_i_winner(),winner_response.get_check_draw_result());
    }
}



