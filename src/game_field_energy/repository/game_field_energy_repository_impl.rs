use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_field_energy::entity::game_field_energy::GameFieldEnergy;
use crate::game_field_energy::repository::game_field_energy_repository::GameFieldEnergyRepository;

pub struct GameFieldEnergyRepositoryImpl {
    game_field_energy_map: IndexMap<i32, GameFieldEnergy>,
}

impl GameFieldEnergyRepositoryImpl {
    pub fn new() -> Self {
        GameFieldEnergyRepositoryImpl {
            game_field_energy_map: IndexMap::new(),
        }
    }

    pub fn get_game_field_energy_map(&mut self) -> &mut IndexMap<i32, GameFieldEnergy> {
        &mut self.game_field_energy_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldEnergyRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldEnergyRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameFieldEnergyRepository for GameFieldEnergyRepositoryImpl {
    fn create_field_energy_object(&mut self, account_unique_id: i32) -> bool {
        println!("FieldEnergyRepositoryImpl: create_field_energy_object()");

        let new_game_field_energy_map = GameFieldEnergy::new(1);
        self.game_field_energy_map.insert(account_unique_id, new_game_field_energy_map);

        true
    }

    fn add_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool {
        println!("FieldEnergyRepositoryImpl: add_field_energy()");

        if let Some(game_field_energy) = self.game_field_energy_map.get_mut(&account_unique_id) {
            for _ in 0..amount {
                game_field_energy.add_energy_count();
            }
            return true
        }

        false
    }

    fn remove_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool {
        println!("FieldEnergyRepositoryImpl: remove_field_energy_with_amount()");

        if let Some(game_field_energy) = self.game_field_energy_map.get_mut(&account_unique_id) {
            for _ in 0..amount {
                game_field_energy.remove_energy_count();
            }
            return true
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_create_field_energy_object() {
        let repository = GameFieldEnergyRepositoryImpl::new();
        let instance = Arc::new(AsyncMutex::new(repository));

        let mut guard = instance.lock().await;
        let account_unique_id = 1;
        let result = guard.create_field_energy_object(account_unique_id);

        assert!(result);

        let energy_map = guard.get_game_field_energy_map();
        if let Some(energy) = energy_map.get(&account_unique_id) {
            println!("Energy count for account {}: {}", account_unique_id, energy.get_energy_count());
        } else {
            println!("Energy count not found for account {}", account_unique_id);
        }

        assert_eq!(
            guard.get_game_field_energy_map().get(&account_unique_id),
            Some(&GameFieldEnergy::new(1))
        );
    }

    #[tokio::test]
    async fn test_increment_and_decrement_of_field_energy() {
        let repository = GameFieldEnergyRepositoryImpl::new();
        let instance = Arc::new(AsyncMutex::new(repository));

        let mut guard = instance.lock().await;
        let account_unique_id = 1;
        let result = guard.create_field_energy_object(account_unique_id);

        assert!(result);

        guard.add_field_energy_with_amount(account_unique_id, 3);

        println!("{:?}", guard.get_game_field_energy_map());

        assert_eq!(
            guard.get_game_field_energy_map().get(&account_unique_id),
            Some(&GameFieldEnergy::new(4))
        );

        guard.remove_field_energy_with_amount(account_unique_id, 2);

        println!("{:?}", guard.get_game_field_energy_map());

        assert_eq!(
            guard.get_game_field_energy_map().get(&account_unique_id),
            Some(&GameFieldEnergy::new(2))
        );
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance1 = GameFieldEnergyRepositoryImpl::get_instance();
        let instance2 = GameFieldEnergyRepositoryImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }
}
