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
use crate::game_deck::service::request::game_deck_card_redraw_request::GameDeckCardRedrawRequest;
use crate::game_deck::service::request::game_deck_card_shuffled_list_request::GameDeckCardShuffledListRequest;
use crate::game_deck::service::response::found_card_from_deck_response::FoundCardFromDeckResponse;
use crate::game_deck::service::response::game_deck_card_draw_list_response::GameDeckCardDrawListResponse;
use crate::game_deck::service::response::game_deck_card_redraw_response::GameDeckCardRedrawResponse;
use crate::game_deck::service::response::game_deck_card_shuffled_list_response::GameDeckCardShuffledListResponse;
use crate::game_deck::service::response::game_start_deck_card_list_response::GameStartDeckCardListResponse;
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

    async fn get_account_unique_id(&self, game_deck_card_list_request: &GameDeckCardListRequest) -> i32 {
        self.parse_account_unique_id(game_deck_card_list_request.get_session_id()).await
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

    async fn shuffle_game_deck(&self, account_unique_id: i32) {
        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        game_deck_repository_guard.shuffle_game_deck(account_unique_id);
        drop(game_deck_repository_guard);
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
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameStartDeckCardListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");

        let account_unique_id = self.get_account_unique_id(&game_deck_card_list_request).await;
        let deck_id = game_deck_card_list_request.get_deck_id();

        self.initialize_game_deck(account_unique_id, deck_id).await;
        self.shuffle_game_deck(account_unique_id).await;

        let drawn_card_list = self.draw_deck_cards(account_unique_id, 5).await;
        let drawn_card_list_clone = drawn_card_list.clone();

        self.add_drawn_cards_to_hand(account_unique_id, drawn_card_list).await;

        GameStartDeckCardListResponse::new(drawn_card_list_clone)
    }

    async fn shuffle_deck(&self, game_deck_card_shuffled_list_request: GameDeckCardShuffledListRequest) -> GameDeckCardShuffledListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");
        let session_id = game_deck_card_shuffled_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        game_deck_repository_guard.shuffle_game_deck(account_unique_id);
        let game_deck_card_vector = game_deck_repository_guard.get_game_deck_card_ids(account_unique_id);

        GameDeckCardShuffledListResponse::new(game_deck_card_vector)
    }

    async fn draw_deck(&self, game_deck_card_draw_request: GameDeckCardDrawRequest) -> GameDeckCardDrawListResponse {
        println!("GameDeckServiceImpl: draw_deck()");

        let session_id = game_deck_card_draw_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let draw_count = game_deck_card_draw_request.get_draw_count();

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let draw_card_vector = game_deck_repository_guard.draw_deck_card(account_unique_id, draw_count);

        GameDeckCardDrawListResponse::new(draw_card_vector)
    }
    async fn shuffle_and_redraw_deck(&self, game_deck_card_redraw_request: GameDeckCardRedrawRequest) -> GameDeckCardRedrawResponse {
        println!("GameDeckServiceImpl: shuffle_and_redraw_deck()");

        let session_id = game_deck_card_redraw_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;

        let draw_count = game_deck_card_redraw_request.get_redraw_card_count();

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;

        game_deck_repository_guard.shuffle_game_deck(account_unique_id);

        let redrawn_card_vector = game_deck_repository_guard.draw_deck_card(account_unique_id, draw_count);
        let remaining_deck_vector = self.get_game_deck_card_ids(account_unique_id).await;

        GameDeckCardRedrawResponse::new(redrawn_card_vector, remaining_deck_vector)
    }

    async fn find_by_card_id_with_count(&self, found_card_from_deck_request: FoundCardFromDeckRequest) -> FoundCardFromDeckResponse {
        println!("GameDeckServiceImpl: shuffle_and_redraw_deck()");

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let found_card_list = game_deck_repository_guard.find_by_card_id_with_count(
            found_card_from_deck_request.get_account_unique_id(),
            found_card_from_deck_request.get_need_to_find_card_id(),
            found_card_from_deck_request.get_energy_count());

        FoundCardFromDeckResponse::new(found_card_list)
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
}