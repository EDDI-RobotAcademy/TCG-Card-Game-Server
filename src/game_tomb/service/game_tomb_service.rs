use async_trait::async_trait;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_tomb::service::response::place_to_tomb_response::PlaceToTombResponse;

#[async_trait]
pub trait GameTombService {
    async fn add_used_card_to_tomb(&mut self, place_to_tomb_request: PlaceToTombRequest) -> PlaceToTombResponse;
}