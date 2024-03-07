use std::sync::Arc;
use diesel::IntoSql;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_winner_check::entity::finish_position::GameFinishPosition;
use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum;

use crate::game_winner_check::repository::game_winner_check_repository::GameWinnerCheckRepository;

pub struct GameWinnerCheckRepositoryImpl {
    game_finish_position_map: IndexMap<i32, FinishPositionEnum>,
}

impl GameWinnerCheckRepositoryImpl {
    pub fn new() -> Self {
        GameWinnerCheckRepositoryImpl {
            game_finish_position_map: IndexMap::new(),
        }
    }

    pub fn get_game_finish_position_map(&mut self) -> &mut IndexMap<i32, FinishPositionEnum> {
        &mut self.game_finish_position_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameWinnerCheckRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameWinnerCheckRepositoryImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        GameWinnerCheckRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameWinnerCheckRepository for GameWinnerCheckRepositoryImpl {
    fn create_finish_position_object(&mut self, account_unique_id: i32, finish_position_enum: FinishPositionEnum) -> bool {
        println!("GameWinnerCheckRepositoryImpl: create_finish_position_object()");

        let new_finish_position = GameFinishPosition::new(finish_position_enum);
        self.game_finish_position_map.insert(account_unique_id, new_finish_position);

        true
    }

    fn add_finish_position_object(&mut self, account_unique_id: i32, finish_position_enum: FinishPositionEnum) -> bool {
        println!("GameWinnerCheckRepositoryImpl: add_finish_position_object()");

        let add_finish_position = GameFinishPosition::new(finish_position_enum);
        self.game_finish_position_map.insert(account_unique_id, add_finish_position);

        true
    }

    fn get_finish_position_enum(&mut self, account_unique_id: i32) -> Option<&FinishPositionEnum> {
        println!("GameWinnerCheckRepository: get_finish_position_enum()");

        if let Some(index) = self.game_finish_position_map.get_index_of(&account_unique_id) {
            if let Some((_key, finish_position_enum)) = &self.game_finish_position_map.get_index_mut(index) {
                let finish_position_enum = self.game_finish_position_map.get(&account_unique_id);
                return finish_position_enum

            }
        }
        None
    }

    fn remove_finish_position_by_account_id(&mut self, account_unique_id: i32) -> bool {
        println!("GameWinnerCheckRepository: remove_finish_position_by_account_id()");

        if let Some(_) = self.game_finish_position_map.get_index_of(&account_unique_id) {
            self.game_finish_position_map.swap_remove(&account_unique_id);

            return true
        }

        false
    }
}

#[cfg(test)]
mod cfg_test {
    use super::*;
    use tokio::test;
    use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum::{Draw, Loser, Winner};
    use crate::game_winner_check::repository::game_winner_check_repository_impl::GameWinnerCheckRepositoryImpl;

    #[test]
    async fn async_test_create_finish_position_object() {
        let mut repository = GameWinnerCheckRepositoryImpl::new();
        let account_unique_id_1 = 456;
        let finish_position_1 = Winner;

        let account_unique_id_2 = 789;
        let finish_position_2 = Loser;

        let result = repository.create_finish_position_object(account_unique_id_1, finish_position_1);
        repository.add_finish_position_object(account_unique_id_2, finish_position_2);

        assert!(result);
        let finish_position_1_option = repository.game_finish_position_map.get(&account_unique_id_1);
        let finish_position_2 = repository.game_finish_position_map.get(&account_unique_id_2).unwrap();
        assert!(finish_position_1_option.is_some());

        if let Some(finish_position) = finish_position_1_option {
            println!("finish position inserted successfully!");
            println!("finish position Value Of account unique id 1: {:?}", finish_position);
            println!("finish position Value Of account unique id 2: {:?}", finish_position_2);
        } else {
            println!("Failed to insert Finish position!");
        }
    }

    #[test]
    async fn test_get_finish_position_enum() {
        let mut repository = GameWinnerCheckRepositoryImpl::new();

        let account_unique_id_1 = 456;
        let finish_position_1 = Loser;

        let account_unique_id_2 = 789;
        let finish_position_2 = Draw;

        repository.create_finish_position_object(account_unique_id_1, finish_position_1);
        repository.add_finish_position_object(account_unique_id_2, finish_position_2);

        let position_enum_account_unique_id_1 = repository.get_finish_position_enum(account_unique_id_1).unwrap();
        println!("finish position Value Of account unique id 1: {:?}", position_enum_account_unique_id_1);

        let position_enum_account_unique_id_2 = repository.get_finish_position_enum(account_unique_id_2).unwrap();
        println!("finish position Value Of account unique id 2: {:?}", position_enum_account_unique_id_2);
    }

    #[test]
    async fn test_remove_finish_position_by_account_id() {
        let mut repository = GameWinnerCheckRepositoryImpl::new();
        let account_unique_id_1 = 456;
        let finish_position_1 = Winner;

        let account_unique_id_2 = 789;
        let finish_position_2 = Loser;

        let result = repository.create_finish_position_object(account_unique_id_1, finish_position_1);
        repository.add_finish_position_object(account_unique_id_2, finish_position_2);

        assert!(result);
        let finish_position_1_option = repository.game_finish_position_map.get(&account_unique_id_1);
        let finish_position_2 = repository.game_finish_position_map.get(&account_unique_id_2).unwrap();
        assert!(finish_position_1_option.is_some());

        if let Some(finish_position) = finish_position_1_option {
            println!("finish position inserted successfully!");
            println!("finish position Value Of account unique id 1: {:?}", finish_position);
            println!("finish position Value Of account unique id 2: {:?}", finish_position_2);

        } else {
            println!("Failed to insert Finish position!");
        }
        println!("game_finish_position_map_before_remove: {:?}", repository.game_finish_position_map);
        repository.remove_finish_position_by_account_id(account_unique_id_1);
        println!("game_finish_position_map_after_remove: {:?}", repository.game_finish_position_map);
        repository.remove_finish_position_by_account_id(account_unique_id_2);
        println!("game_finish_position_map_after_remove: {:?}", repository.game_finish_position_map);
    }
}