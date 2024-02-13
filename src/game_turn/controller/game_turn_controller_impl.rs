use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service::FirstTurnDecisionWaitQueueService;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service_impl::FirstTurnDecisionWaitQueueServiceImpl;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;

use crate::game_turn::controller::game_turn_controller::GameTurnController;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::request_form::first_turn_decision_wait_queue_request_form:: FirstTurnDecisionWaitQueueRequestForm;
use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;
use crate::game_turn::controller::response_form::first_turn_decision_wait_queue_response_form::FirstTurnDecisionWaitQueueResponseForm;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::game_turn::service::game_turn_service::GameTurnService;

use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameTurnControllerImpl {
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    // TODO: Need Refactor
    first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>,

}

impl GameTurnControllerImpl {
    pub fn new(game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               // TODO: Need Refactor
               first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>) -> Self {

        GameTurnControllerImpl {
            game_turn_service,
            game_field_unit_service,
            redis_in_memory_service,
            // TODO: Need Refactor
            first_turn_decision_wait_queue_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameTurnControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnControllerImpl::new(
                            GameTurnServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            // TODO: Need Refactor
                            FirstTurnDecisionWaitQueueServiceImpl::get_instance())));
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
}

#[async_trait]
impl GameTurnController for GameTurnControllerImpl {
    // TODO: Need Refactor
    async fn execute_first_turn_decision_wait_queue_procedure(&self, first_turn_decision_wait_queue_request_form: FirstTurnDecisionWaitQueueRequestForm) ->
                                                                                            FirstTurnDecisionWaitQueueResponseForm {
        println!("GameTurnControllerImpl: execute_first_turn_decision_procedure()");
        let session_id=first_turn_decision_wait_queue_request_form.get_session_id().to_string();
        let choice=first_turn_decision_wait_queue_request_form.get_choice().to_string();
        let request = FirstTurnDecisionWaitQueueRequest::new(session_id, choice);


        let mut first_turn_decision_wait_queue_service_guard = self.first_turn_decision_wait_queue_service.lock().await;
        let response=first_turn_decision_wait_queue_service_guard.enqueue_player_tuple_to_wait_queue(request).await;
        drop(first_turn_decision_wait_queue_service_guard);
        if response.get_is_success() == false {
            return FirstTurnDecisionWaitQueueResponseForm::new(false)
        }
        return FirstTurnDecisionWaitQueueResponseForm::new(true);

    }

    // TODO: Need Refactor
    async fn execute_first_turn_decision_procedure(&self, first_turn_decision_request_form: FirstTurnDecisionRequestForm)
                                                                                         -> FirstTurnDecisionResponseForm {
        let session_id=first_turn_decision_request_form.get_session_id().to_string();
        let request=FirstTurnDecisionRequest::new(session_id);
        let mut game_turn_service_guard=self.game_turn_service.lock().await;
        let response=game_turn_service_guard.first_turn_decision_object(request).await;
        drop(game_turn_service_guard);
        return FirstTurnDecisionResponseForm::new(response.clone().get_first_player(),response.get_am_i_first_player(),response.get_result_is_draw());
    }

    async fn request_turn_end(&self, turn_end_request_form: TurnEndRequestForm) -> TurnEndResponseForm {
        // 1. Redis에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(turn_end_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            println!("Invalid session");
            return TurnEndResponseForm::new(false)
        }

        // TODO: 2. 현재 요청한 사람이 이번 턴의 주도권을 가지고 있던 사람인지 검증

        // 3. 자신의 필드 유닛들 중 턴 종료 시 데미지를 받는 케이스를 적용 (현재 상황에서 화상 데미지)
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let apply_status_effect_damage_iteratively_response = game_field_unit_service_guard.apply_status_effect_damage_iteratively(
            turn_end_request_form.to_apply_status_effect_damage_iteratively_request(
                account_unique_id)).await;

        // 4. 죽은 유닛들이 있는지 전체 순회하며 확인

        // 5. 죽은 유닛들 무덤으로 배치

        // 6. 양측 턴을 증가시키기 위해 opponent id도 찾아야함

        // 7. 당신의 턴 증가

        // 8. 상대방의 턴 증가

        // 9. 턴 종료 상황에서 상태 이상으로 죽은 유닛들, 데미지 등등을 알려줘야함

        TurnEndResponseForm::new(true)
    }
}