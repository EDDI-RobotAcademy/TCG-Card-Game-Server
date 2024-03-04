use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;

use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::controller::request_form::check_rock_paper_scissors_winner_request_form::CheckRockPaperScissorsWinnerRequestForm;
use crate::rock_paper_scissors::controller::request_form::rock_paper_scissors_request_form::RockPaperScissorsRequestForm;
use crate::rock_paper_scissors::controller::response_form::check_rock_paper_scissors_winner_response_form::CheckRockPaperScissorsWinnerResponseForm;
use crate::rock_paper_scissors::controller::response_form::rock_paper_scissors_response_form::RockPaperScissorsResponseForm;
use crate::rock_paper_scissors::controller::rock_paper_scissors_controller::RockPaperScissorsController;
use crate::rock_paper_scissors::service::request::check_opponent_choice_request::CheckOpponentChoiceRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::CheckRockPaperScissorsWinnerRequest;
use crate::rock_paper_scissors::service::request::register_rock_paper_scissors_wait_hash_request::RegisterRockPaperScissorsWaitHashRequest;
use crate::rock_paper_scissors::service::rock_paper_scissors_service::RockPaperScissorsService;
use crate::rock_paper_scissors::service::rock_paper_scissors_service_impl::{RockPaperScissorsServiceImpl};

pub struct RockPaperScissorsControllerImpl {
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,
}

impl RockPaperScissorsControllerImpl {
    pub fn new(battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               rock_paper_scissors_service: Arc<AsyncMutex<RockPaperScissorsServiceImpl>>,) -> Self {

        RockPaperScissorsControllerImpl {
            battle_room_service,
            game_protocol_validation_service,
            redis_in_memory_service,
            rock_paper_scissors_service,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockPaperScissorsControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockPaperScissorsControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RockPaperScissorsControllerImpl::new(
                            BattleRoomServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            RockPaperScissorsServiceImpl::get_instance())));
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
    async fn get_opponent_unique_id(&self, find_opponent_by_account_id_request: FindOpponentByAccountIdRequest) -> i32 {
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(find_opponent_by_account_id_request).await;
        drop(battle_room_service_guard);
        find_opponent_by_account_id_response.get_opponent_unique_id()
    }
}

#[async_trait]
impl RockPaperScissorsController for RockPaperScissorsControllerImpl {
    async fn execute_rock_paper_scissors_procedure(
        &self, rock_paper_scissors_request_form: RockPaperScissorsRequestForm)
        -> RockPaperScissorsResponseForm {

        println!("RockPaperScissorsControllerImpl: execute_rock_paper_scissors_procedure()");

        let account_unique_id =
            self.is_valid_session(
                rock_paper_scissors_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return RockPaperScissorsResponseForm::new(false)
        }

        let opponent_unique_id =
            self.get_opponent_unique_id(
                rock_paper_scissors_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await;

        let mut rock_paper_scissors_service_guard =
            self.rock_paper_scissors_service.lock().await;

        let wait_queue_response=
            rock_paper_scissors_service_guard.register_rock_paper_scissors_wait_hash(
                rock_paper_scissors_request_form
                    .to_register_rock_paper_scissors_wait_hash_request(
                        account_unique_id,
                        opponent_unique_id,
                        rock_paper_scissors_request_form.get_choice())).await;

        if wait_queue_response.get_is_success() {
            println!("{}번 계정 가위바위보 정보 등록 완료", account_unique_id);
        }

        RockPaperScissorsResponseForm::new(wait_queue_response.get_is_success())
    }

    async fn execute_check_rock_paper_scissors_winner_procedure(
        &self, check_winner_rock_paper_scissors_request_form: CheckRockPaperScissorsWinnerRequestForm)
        -> CheckRockPaperScissorsWinnerResponseForm {

        println!("RockPaperScissorsControllerImpl: execute_check_rock_paper_scissors_winner_procedure()");

        let account_unique_id =
            self.is_valid_session(
                check_winner_rock_paper_scissors_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return CheckRockPaperScissorsWinnerResponseForm::new("WAIT".to_string())
        }

        let opponent_unique_id =
            self.get_opponent_unique_id(
                check_winner_rock_paper_scissors_request_form
                    .to_find_opponent_by_account_id_request(account_unique_id)).await;

        let mut rock_paper_scissors_service_guard =
            self.rock_paper_scissors_service.lock().await;

        let has_opponent_choice =
            rock_paper_scissors_service_guard.check_opponent_choice(
                check_winner_rock_paper_scissors_request_form
                    .to_check_opponent_choice_request(opponent_unique_id)).await.get_opponent_check();

        if !has_opponent_choice {
            println!("상대가 아직 가위바위보를 실행하지 않았습니다.");
            return CheckRockPaperScissorsWinnerResponseForm::new("WAIT".to_string())
        }

        let check_rock_paper_scissors_winner_response =
            rock_paper_scissors_service_guard.check_rock_paper_scissors_winner(
                check_winner_rock_paper_scissors_request_form
                    .to_check_rock_paper_scissors_winner_request(account_unique_id, opponent_unique_id)).await;

        drop(rock_paper_scissors_service_guard);

        CheckRockPaperScissorsWinnerResponseForm::new(
            check_rock_paper_scissors_winner_response.get_am_i_winner().to_string())
    }
}



