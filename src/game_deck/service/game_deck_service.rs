use async_trait::async_trait;
use crate::game_deck::service::request::found_card_from_deck_request::FoundCardFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_draw_request::GameDeckCardDrawRequest;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_deck::service::request::game_deck_start_card_list_request::{GameDeckStartCardListRequest};
use crate::game_deck::service::request::game_deck_card_shuffle_request::{GameDeckCardShuffleRequest};
use crate::game_deck::service::request::search_specific_deck_card_request::SearchSpecificDeckCardRequest;
use crate::game_deck::service::response::found_card_from_deck_response::FoundCardFromDeckResponse;
use crate::game_deck::service::response::game_deck_card_draw_list_response::GameDeckCardDrawListResponse;
use crate::game_deck::service::response::game_deck_card_list_response::GameDeckCardListResponse;
use crate::game_deck::service::response::game_deck_card_shuffle_response::{GameDeckCardShuffleResponse};
use crate::game_deck::service::response::game_deck_start_card_list_response::{GameDeckStartCardListResponse};
use crate::game_deck::service::response::search_specific_deck_card_response::SearchSpecificDeckCardResponse;

#[async_trait]
pub trait GameDeckService {
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckStartCardListRequest) -> GameDeckStartCardListResponse;
    async fn shuffle_deck(&self, game_deck_card_shuffle_request: GameDeckCardShuffleRequest) -> GameDeckCardShuffleResponse;

    // TODO: 드로우와 관련된 코드는 draw_cards_from_deck 으로 교체 필요
    async fn draw_deck(&self, game_deck_card_draw_request: GameDeckCardDrawRequest) -> GameDeckCardDrawListResponse;
    async fn draw_cards_from_deck(&self, game_deck_card_draw_request: GameDeckCardDrawRequest) -> GameDeckCardDrawListResponse;
    async fn get_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameDeckCardListResponse;
    async fn find_by_card_id_with_count(&self, found_card_from_deck_request: FoundCardFromDeckRequest) -> FoundCardFromDeckResponse;
    async fn search_specific_deck_card(&self, search_specific_deck_card_request: SearchSpecificDeckCardRequest) -> SearchSpecificDeckCardResponse;
}