use async_trait::async_trait;

use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;

#[async_trait]
pub trait  GameTurnController {

    async fn request_turn_end(
        &self, turn_end_request_form: TurnEndRequestForm) -> TurnEndResponseForm;
}