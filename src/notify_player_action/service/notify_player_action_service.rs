use async_trait::async_trait;
use crate::notify_player_action::service::request::notify_to_opponent_what_you_do_request::NotifyToOpponentWhatYouDoRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_energy_card_request::NotifyToOpponentYouUseEnergyCardRequest;
use crate::notify_player_action::service::response::notify_to_opponent_what_you_do_response::NotifyToOpponentWhatYouDoResponse;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_energy_card_response::NotifyToOpponentYouUseEnergyCardResponse;

#[async_trait]
pub trait NotifyPlayerActionService {
    async fn notify_to_opponent_what_you_do(&mut self, notify_to_opponent_what_you_do_request: NotifyToOpponentWhatYouDoRequest) -> NotifyToOpponentWhatYouDoResponse;
    async fn notify_to_opponent_you_use_energy_card(&mut self, notify_to_opponent_you_use_energy_card_request: NotifyToOpponentYouUseEnergyCardRequest) -> NotifyToOpponentYouUseEnergyCardResponse;
}
