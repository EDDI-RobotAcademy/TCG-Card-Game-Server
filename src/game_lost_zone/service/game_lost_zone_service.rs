use async_trait::async_trait;
use crate::game_lost_zone::service::request::place_card_to_lost_zone_request::PlaceCardToLostZoneRequest;
use crate::game_lost_zone::service::response::place_card_to_lost_zone_response::PlaceCardToLostZoneResponse;

#[async_trait]
pub trait GameLostZoneService {
    async fn place_card_to_lost_zone(&self, place_card_to_lost_zone_request: PlaceCardToLostZoneRequest) -> PlaceCardToLostZoneResponse;
}