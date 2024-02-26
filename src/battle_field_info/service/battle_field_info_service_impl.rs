use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use serde_json::to_string;
use tokio::sync::Mutex as AsyncMutex;
use crate::battle_field_info::service::battle_field_info_service::BattleFieldInfoService;
use crate::battle_field_info::service::request::remain_deck_card_count_request::{RemainDeckCardCountRequest};
use crate::battle_field_info::service::response::remain_deck_card_count_response::{RemainDeckCardCountResponse};
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;
use crate::battle_room::repository::battle_room_repository_impl::BattleRoomRepositoryImpl;
use crate::game_deck::repository::game_deck_repository::GameDeckRepository;
use crate::game_deck::repository::game_deck_repository_impl::GameDeckRepositoryImpl;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;


pub struct BattleFieldInfoServiceImpl {
    battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
    game_deck_repository:   Arc<AsyncMutex<GameDeckRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>
}

impl BattleFieldInfoServiceImpl {
    pub fn new(battle_room_repository: Arc<AsyncMutex<BattleRoomRepositoryImpl>>,
               game_deck_repository:   Arc<AsyncMutex<GameDeckRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,) -> Self {
        BattleFieldInfoServiceImpl {
            battle_room_repository,
            game_deck_repository,
            redis_in_memory_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleFieldInfoServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleFieldInfoServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleFieldInfoServiceImpl::new(
                            BattleRoomRepositoryImpl::get_instance(),
                            GameDeckRepositoryImpl::get_instance(),
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
impl BattleFieldInfoService for BattleFieldInfoServiceImpl {
    async fn get_remain_deck_card_count(&mut self, get_my_remain_deck_card_count_request: RemainDeckCardCountRequest) -> RemainDeckCardCountResponse {
        println!("BattleFieldInfoServiceImpl: get_remain_deck_card_list()");
        let account_unique_id = self.parse_account_unique_id(get_my_remain_deck_card_count_request.get_session_id()).await;
        let battle_room_repository_guard=self.battle_room_repository.lock().await;
        let opponent_id= battle_room_repository_guard.find_opponent_unique_id(account_unique_id).await.unwrap();

        let mut game_deck_repository_guard = self.game_deck_repository.lock().await;
        if get_my_remain_deck_card_count_request.get_who()=="me"
        {
            let mut count=game_deck_repository_guard.get_remain_deck_card_count(account_unique_id);
            RemainDeckCardCountResponse::new(count)
        }
        else
        {
            let mut opponent_count=game_deck_repository_guard.get_remain_deck_card_count(opponent_id);
            RemainDeckCardCountResponse::new(opponent_count)
        }

    }
}
