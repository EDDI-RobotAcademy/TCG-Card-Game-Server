use async_trait::async_trait;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_deck::service::request::game_deck_card_shuffled_list_request::GameDeckCardShuffledListRequest;
use crate::game_deck::service::response::game_deck_card_shuffled_list_response::GameDeckCardShuffledListResponse;

#[async_trait]
pub trait GameDeckService {
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameDeckCardShuffledListResponse;
    async fn shuffle_deck(&self, game_deck_card_list_request: GameDeckCardShuffledListRequest) -> GameDeckCardShuffledListResponse;
}