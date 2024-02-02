use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;
use crate::account_deck_card::repository::account_deck_card_repository_impl::AccountDeckCardRepositoryImpl;
use crate::common::converter::hash_to_vector_converter::HashToVectorConverter;
use crate::game_deck::entity::game_deck_card::GameDeckCard;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;

use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::request::game_deck_card_draw_request::GameDeckCardDrawRequest;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_deck::service::request::game_deck_card_shuffled_list_request::GameDeckCardShuffledListRequest;
use crate::game_deck::service::response::game_deck_card_draw_list_response::GameDeckCardDrawListResponse;
use crate::game_deck::service::response::game_deck_card_shuffled_list_response::GameDeckCardShuffledListResponse;
use crate::game_deck::service::response::game_start_deck_card_list_response::GameStartDeckCardListResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct GameDeckServiceImpl {
    game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl GameDeckServiceImpl {
    pub fn new(game_deck_repository: Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>) -> Self {
        GameDeckServiceImpl {
            game_deck_repository,
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
}

#[async_trait]
impl GameDeckService for GameDeckServiceImpl {
    async fn create_and_shuffle_deck(&self, game_deck_card_list_request: GameDeckCardListRequest) -> GameStartDeckCardListResponse {
        println!("GameDeckServiceImpl: create_and_shuffle_deck()");

        let session_id = game_deck_card_list_request.get_session_id();
        let account_unique_id = self.parse_account_unique_id(session_id).await;
        let deck_id = game_deck_card_list_request.get_deck_id();

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

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        game_deck_repository_guard.shuffle_game_deck(account_unique_id);
        drop(game_deck_repository_guard);

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let drawn_card_list = game_deck_repository_guard.draw_deck_card(account_unique_id, 5);
        drop(game_deck_repository_guard);

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        let game_deck_card_vector = game_deck_repository_guard.get_game_deck_card_ids(account_unique_id);
        drop(game_deck_repository_guard);

        GameStartDeckCardListResponse::new(drawn_card_list, game_deck_card_vector)
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