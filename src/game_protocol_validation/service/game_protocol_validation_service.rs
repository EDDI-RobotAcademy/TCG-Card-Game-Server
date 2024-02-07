use async_trait::async_trait;
use crate::game_protocol_validation::service::request::support_card_protocol_validation_request::SupportCardProtocolValidationRequest;
use crate::game_protocol_validation::service::response::support_card_protocol_validation_response::SupportCardProtocolValidationResponse;

#[async_trait]
pub trait GameProtocolValidationService {
    async fn support_card_protocol_validation(&mut self, support_card_validation_request: SupportCardProtocolValidationRequest) -> SupportCardProtocolValidationResponse;
}