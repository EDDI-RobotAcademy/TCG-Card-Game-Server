use async_trait::async_trait;
use crate::notify_player_action::service::request::notify_to_opponent_what_you_do_request::NotifyToOpponentWhatYouDoRequest;
use crate::notify_player_action::service::response::notify_to_opponent_what_you_do_response::NotifyToOpponentWhatYouDoResponse;

#[async_trait]
pub trait NotifyPlayerActionService {
    async fn notify_to_opponent_what_you_do(&mut self, notify_to_opponent_what_you_do_request: NotifyToOpponentWhatYouDoRequest) -> NotifyToOpponentWhatYouDoResponse;
}