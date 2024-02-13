use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service::FirstTurnDecisionWaitQueueService;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service_impl::FirstTurnDecisionWaitQueueServiceImpl;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;

use crate::game_turn::controller::game_turn_controller::GameTurnController;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::request_form::first_turn_decision_wait_queue_request_form:: FirstTurnDecisionWaitQueueRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;
use crate::game_turn::controller::response_form::first_turn_decision_wait_queue_response_form::FirstTurnDecisionWaitQueueResponseForm;
use crate::game_turn::service::game_turn_service::GameTurnService;

use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;

pub struct GameTurnControllerImpl {
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>,

}

impl GameTurnControllerImpl {
    pub fn new(game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>) -> Self {
        GameTurnControllerImpl {
            game_turn_service,
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
                            FirstTurnDecisionWaitQueueServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameTurnController for GameTurnControllerImpl {
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

    async fn execute_first_turn_decision_procedure(&self, first_turn_decision_request_form: FirstTurnDecisionRequestForm)
                                                                                         -> FirstTurnDecisionResponseForm {
        let session_id=first_turn_decision_request_form.get_session_id().to_string();
        let request=FirstTurnDecisionRequest::new(session_id);
        let mut game_turn_service_guard=self.game_turn_service.lock().await;
        let response=game_turn_service_guard.first_turn_decision_object(request).await;
        drop(game_turn_service_guard);
        return FirstTurnDecisionResponseForm::new(response.clone().get_first_player(),response.get_am_i_first_player(),response.get_result_is_draw());
    }
}