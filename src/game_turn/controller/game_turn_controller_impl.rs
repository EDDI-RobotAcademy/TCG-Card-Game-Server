use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_turn::controller::game_turn_controller::GameTurnController;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;
use crate::game_turn::service::game_turn_service_impl::GameTurnServiceImpl;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service_impl::FirstTurnDecisionWaitQueueServiceImpl;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::game_turn::service::game_turn_service::GameTurnService;
use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;
use crate::first_turn_decision_wait_queue::service::first_turn_decision_wait_queue_service::FirstTurnDecisionWaitQueueService;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;
use crate::game_turn::service::request::decide_first_turn_request::DecideFirstTurnRequest;


pub struct GameTurnControllerImpl {
    game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
    first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>,


}

impl GameTurnControllerImpl {
    pub fn new(game_turn_service: Arc<AsyncMutex<GameTurnServiceImpl>>,
               first_turn_decision_wait_queue_service: Arc<AsyncMutex<FirstTurnDecisionWaitQueueServiceImpl>>, ) -> Self {
        GameTurnControllerImpl {
            game_turn_service,
            first_turn_decision_wait_queue_service,

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
impl From<FirstTurnDecisionRequestForm> for FirstTurnDecisionWaitQueueRequest {
    fn from(a: FirstTurnDecisionRequestForm) -> Self {
        FirstTurnDecisionWaitQueueRequest {
            session_id: a.session_id,
            choice: a.choice,
        }
    }
}
#[async_trait]
impl GameTurnController for GameTurnControllerImpl {

    async fn execute_first_turn_decision_procedure(&self, decide_first_turn_request_form: FirstTurnDecisionRequestForm) -> FirstTurnDecisionResponseForm {
        println!("GameTurnControllerImpl: execute_first_turn_decision_procedure()");


        let mut first_turn_decision_wait_queue_service_guard = self.first_turn_decision_wait_queue_service.lock().await;
        let mut request =decide_first_turn_request_form.into();
        let mut two_players_response=first_turn_decision_wait_queue_service_guard.enqueue_player_session_id_to_wait_queue(request).await;
        drop(first_turn_decision_wait_queue_service_guard);
        let  player_id1=two_players_response.get_session_id1();
        let  player_choice1=two_players_response.get_choice1();
        let  player_id2=two_players_response.get_session_id2();
        let  player_choice2=two_players_response.get_choice2();
        let mut game_turn_service_guard=self.game_turn_service.lock().await;
        let two_players_request=DecideFirstTurnRequest::new(player_id1,player_choice1.to_string(),player_id2,player_choice2.to_string());

        let mut result=game_turn_service_guard.decide_first_turn_object(two_players_request).await;
        drop(game_turn_service_guard);


        FirstTurnDecisionResponseForm::new(result.first_player.to_string(), false)
    }
}
