use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;
use crate::account_deck_card::repository::account_deck_card_repository_impl::AccountDeckCardRepositoryImpl;
use crate::common::converter::hash_to_vector_converter::HashToVectorConverter;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;

use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
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
use crate::game_hand::repository::game_hand_repository::GameHandRepository;
use crate::game_hand::repository::game_hand_repository_impl::GameHandRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameDeckServiceImpl {
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
    account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl GameDeckServiceImpl {
    pub fn new(game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               game_hand_repository: Arc<AsyncMutex<GameHandRepositoryImpl>>,
               account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {

        GameDeckServiceImpl {
            game_deck_repository,
            game_hand_repository,
            account_deck_card_repository,
            redis_in_memory_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameDeckServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameDeckServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameDeckServiceImpl::new(
                            GameDeckRepositoryImpl::get_instance(),
                            GameHandRepositoryImpl::get_instance(),
                            AccountDeckCardRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn parse_account_unique_id(&self, session_id: &str) -> i32 {
        let mut redis_in_memory_repository = self.redis_in_memory_repository.lock().await;
        let account_unique_id_option_string = redis_in_memory_repository.get(session_id).await;
        let account_unique_id_string = account_unique_id_option_string.unwrap();
        account_unique_id_string.parse().expect("Failed to parse account_unique_id_string as i32")
    }

    async fn initialize_game_deck(&self, account_unique_id: i32, deck_id: i32) {
        let account_deck_card_repository_guard = self.account_deck_card_repository.lock().await;
        let account_deck_list = account_deck_card_repository_guard.get_card_list(deck_id).await;

        let account_deck_hash_vector = match account_deck_list {
            Ok(Some(card_list)) => card_list,
            _ => Vec::new(),
        };
        let account_deck_vector = HashToVectorConverter::hash_vector_to_vector(account_deck_hash_vector);
        drop(account_deck_card_repository_guard);

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        game_deck_repository_guard.set_game_deck_from_data(account_unique_id, account_deck_vector);
        drop(game_deck_repository_guard);
    }

    async fn shuffle_game_deck(&self, account_unique_id: i32) -> bool {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        return game_deck_repository_guard.shuffle_game_deck(account_unique_id);
    }

    async fn draw_deck_cards(&self, account_unique_id: i32, num_cards: usize) -> Vec<i32> {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let drawn_card_list = game_deck_repository_guard.draw_deck_card(account_unique_id, num_cards as i32);
        drop(game_deck_repository_guard);

        drawn_card_list
    }

    async fn add_drawn_cards_to_hand(&self, account_unique_id: i32, drawn_card_list: Vec<i32>) {
        let mut game_hand_repository_guard = self.game_hand_repository.lock().await;
        game_hand_repository_guard.add_card_list_to_hand(account_unique_id, drawn_card_list);
        drop(game_hand_repository_guard);
    }

    async fn get_game_deck_card_ids(&self, account_unique_id: i32) -> Vec<i32> {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let game_deck_card_vector = game_deck_repository_guard.get_game_deck_card_ids(account_unique_id);
        drop(game_deck_repository_guard);

        game_deck_card_vector
    }
}

#[async_trait]
impl GameDeckService for GameDeckServiceImpl {
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckStartCardListRequest) -> GameDeckStartCardListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;
        let deck_id = game_deck_card_list_request.get_deck_id();

        self.initialize_game_deck(account_unique_id, deck_id).await;
        self.shuffle_game_deck(account_unique_id).await;

        let drawn_card_list = self.draw_deck_cards(account_unique_id, 5).await;
        let drawn_card_list_clone = drawn_card_list.clone();

        self.add_drawn_cards_to_hand(account_unique_id, drawn_card_list).await;

        GameDeckStartCardListResponse::new(drawn_card_list_clone)
    }

    async fn shuffle_deck(&self, game_deck_card_shuffle_request: GameDeckCardShuffleRequest) -> GameDeckCardShuffleResponse {
        println!("GameDeckServiceImpl: shuffle_deck()");

        let session_id = game_deck_card_shuffle_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let shuffle_result = self.shuffle_game_deck(account_unique_id).await;

        GameDeckCardShuffleResponse::new(shuffle_result)
    }

    async fn draw_deck(&self, game_deck_card_draw_request: GameDeckCardDrawRequest) -> GameDeckCardDrawListResponse {
        println!("GameDeckServiceImpl: draw_deck()");

        let session_id = game_deck_card_draw_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let draw_count: usize = game_deck_card_draw_request.get_draw_count() as usize;

        let draw_card_vector = self.draw_deck_cards(account_unique_id, draw_count).await;

        self.add_drawn_cards_to_hand(account_unique_id, draw_card_vector.clone()).await;

        GameDeckCardDrawListResponse::new(draw_card_vector.clone())
    }

    async fn get_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameDeckCardListResponse {
        println!("GameDeckServiceImpl: get_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let deck_card_list = self.get_game_deck_card_ids(account_unique_id).await;

        GameDeckCardListResponse::new(deck_card_list)
    }

    async fn find_by_card_id_with_count(&self, found_card_from_deck_request: FoundCardFromDeckRequest) -> FoundCardFromDeckResponse {
        println!("GameDeckServiceImpl: find_by_card_id_with_count()");

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let found_card_list = game_deck_repository_guard.find_by_card_id_with_count(
            found_card_from_deck_request.get_account_unique_id(),
            found_card_from_deck_request.get_need_to_find_card_id(),
            found_card_from_deck_request.get_card_count());

        FoundCardFromDeckResponse::new(found_card_list)
    }

    async fn search_specific_deck_card(&self, search_specific_deck_card_request: SearchSpecificDeckCardRequest) -> SearchSpecificDeckCardResponse {
        println!("GameDeckServiceImpl: search_specific_deck_card()");

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;

        let account_unique_id = search_specific_deck_card_request.get_account_unique_id();
        let will_be_found_card_id = search_specific_deck_card_request.get_target_card_id();
        let default_card_count = 1;

        let found_card_list = game_deck_repository_guard
            .find_by_card_id_with_count(account_unique_id, will_be_found_card_id, default_card_count);

        if found_card_list.is_empty() {
            return SearchSpecificDeckCardResponse::new(false);
        }

        self.add_drawn_cards_to_hand(account_unique_id, found_card_list).await;

        SearchSpecificDeckCardResponse::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_game_deck() {
        let deck_id = 1;
        let account_id = 1;

        let account_deck_card_repository_mutex = AccountDeckCardRepositoryImpl::get_instance();
        let account_deck_card_repository_guard = account_deck_card_repository_mutex.lock().await;
        let account_deck_list = account_deck_card_repository_guard.get_card_list(deck_id).await;

        let account_deck_hash_vector = match account_deck_list {
            Ok(Some(card_list)) => card_list,
            _ => Vec::new(),
        };
        let account_deck_vector = HashToVectorConverter::hash_vector_to_vector(account_deck_hash_vector.clone());
        println!("account_deck_list: {:?}", account_deck_hash_vector);

        let game_deck_repository_mutex = GameDeckRepositoryImpl::get_instance();
        let mut game_deck_repository_guard = game_deck_repository_mutex.lock().await;
        game_deck_repository_guard.set_game_deck_from_data(account_id, account_deck_vector);

        let game_deck_map = game_deck_repository_guard.get_game_deck_map();
        println!("Game Deck: {:?}", game_deck_map);

        game_deck_repository_guard.shuffle_game_deck(account_id);
        let game_deck_map = game_deck_repository_guard.get_game_deck_map();
        println!("After Shuffle -> Game Deck: {:?}", game_deck_map);

        let game_deck = game_deck_map.get(&account_id).unwrap();
        let game_deck_vector = game_deck.get_card_ids();

        println!("Game Deck as Vec<i32>: {:?}", game_deck_vector);
    }

    #[test]
    async fn test_search() {
        let game_deck_service_mutex = GameDeckServiceImpl::get_instance();
        let mut game_deck_service = game_deck_service_mutex.lock().await;

        let game_deck_start_card_list_request
            = GameDeckStartCardListRequest::new("1".to_string(), "redis_token_str".to_string());

        game_deck_service.create_and_shuffle_deck(game_deck_start_card_list_request).await;

        let search_request1 = SearchSpecificDeckCardRequest::new(1, 25);
        let search_result1 = game_deck_service.search_specific_deck_card(search_request1).await;

        assert_eq!(true, search_result1.is_success());

        let search_request2 = SearchSpecificDeckCardRequest::new(1, 1000);
        let search_result2 = game_deck_service.search_specific_deck_card(search_request2).await;

        assert_eq!(false, search_result2.is_success());
    }
}