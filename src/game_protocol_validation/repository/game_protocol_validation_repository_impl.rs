use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_protocol_validation::repository::game_protocol_validation_repository::GameProtocolValidationRepository;


pub struct GameProtocolValidationRepositoryImpl {
    // TODO: key: PlayerUniqueId (사용자 account id), value: 조작 회수
    player_cheating_count_map: HashMap<i32, i32>,
}

impl GameProtocolValidationRepositoryImpl {
    pub fn new() -> Self {
        GameProtocolValidationRepositoryImpl {
            player_cheating_count_map: HashMap::new(),
        }
    }

    pub(crate) fn get_game_hand_map(&mut self) -> &mut HashMap<i32, i32> {
        &mut self.player_cheating_count_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameProtocolValidationRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameProtocolValidationRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameProtocolValidationRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameProtocolValidationRepository for GameProtocolValidationRepositoryImpl {

    // TODO: 부정 행위 적발 시 기록
    fn record_cheating_player(&mut self, account_unique_id: i32, support_card_id: i32, current_round: i32) -> bool {
        println!("GameProtocolValidationRepositoryImpl: support_card_protocol_validation()");

        true
    }
}