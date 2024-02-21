use async_trait::async_trait;
use crate::game_tomb::service::request::add_dead_unit_list_to_tomb_request::AddDeadUnitListToTombRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_tomb::service::response::add_dead_unit_list_to_tomb_response::AddDeadUnitListToTombResponse;
use crate::game_tomb::service::response::place_to_tomb_response::PlaceToTombResponse;

#[async_trait]
pub trait GameTombService {
    async fn add_used_card_to_tomb(&mut self, place_to_tomb_request: PlaceToTombRequest) -> PlaceToTombResponse;
    async fn add_dead_unit_to_tomb(&mut self, place_to_tomb_request: PlaceToTombRequest) -> PlaceToTombResponse;
    async fn add_dead_unit_list_to_tomb(&mut self, add_dead_unit_list_to_tomb_request: AddDeadUnitListToTombRequest) -> AddDeadUnitListToTombResponse;
}