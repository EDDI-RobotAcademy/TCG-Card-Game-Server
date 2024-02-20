use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_turn::entity::game_turn::GameTurn;
use crate::game_turn::repository::game_turn_repository::GameTurnRepository;



pub struct GameTurnRepositoryImpl {
    game_turn_map: IndexMap<i32, GameTurn>,
}

impl GameTurnRepositoryImpl {
    pub fn new() -> Self {
        GameTurnRepositoryImpl {
            game_turn_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_turn_map(&mut self) -> &mut IndexMap<i32, GameTurn> {
        &mut self.game_turn_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameTurnRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameTurnRepositoryImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameTurnRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameTurnRepository for GameTurnRepositoryImpl {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameTurnRepositoryImpl: create_game_turn_object()");

        let new_game_round = GameTurn::new();
        self.game_turn_map.insert(account_unique_id, new_game_round);

        true
    }

    fn next_game_turn(&mut self, account_unique_id: i32) -> i32 {
        println!("GameTurnRepositoryImpl: next_game_turn()");

        if let Some(index) = self.game_turn_map.get_index_of(&account_unique_id) {
            if let Some((_key, game_turn)) = self.game_turn_map.get_index_mut(index) {
                game_turn.next_turn();
                return game_turn.get_turn();
            }
        }

        -1
    }

    fn get_game_turn(&mut self, account_unique_id: i32) -> i32 {
        println!("GameTurnRepositoryImpl: get_game_turn()");

        if let Some(index) = self.game_turn_map.get_index_of(&account_unique_id) {
            if let Some((_key, game_turn)) = self.game_turn_map.get_index_mut(index) {
                return game_turn.get_turn();
            }
        }

        -1
    }

}

#[cfg(test)]
mod cfg_test {

    use rand::{Rng, SeedableRng};

    fn generate_random_bool(seed: u64) -> bool {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        rng.gen::<bool>()
    }

    #[test]
    fn test_random() {
        let player1_seed = 123; // 원하는 시드 값
        let player2_seed = 456; // 다른 시드 값

        // 각 플레이어의 랜덤 결과 생성
        let player1_result = generate_random_bool(player1_seed);
        let player2_result = generate_random_bool(player2_seed);

        // 결과 출력
        println!("Player 1 Result: {}", player1_result);
        println!("Player 2 Result: {}", player2_result);
    }


}