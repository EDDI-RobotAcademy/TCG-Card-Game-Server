use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_field_energy::entity::game_field_energy::GameFieldEnergy;
use crate::game_field_energy::repository::field_energy_repository::FieldEnergyRepository;

pub struct FieldEnergyRepositoryImpl {
    game_field_energy_map: IndexMap<i32, GameFieldEnergy>,
}

impl FieldEnergyRepositoryImpl {
    pub fn new() -> Self {
        FieldEnergyRepositoryImpl {
            game_field_energy_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_field_energy_map(&mut self) -> &mut IndexMap<i32, GameFieldEnergy> {
        &mut self.game_field_energy_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<FieldEnergyRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FieldEnergyRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FieldEnergyRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl FieldEnergyRepository for FieldEnergyRepositoryImpl {
    fn create_field_energy_object(&mut self, account_unique_id: i32) -> bool {
        println!("FieldEnergyRepositoryImpl: create_field_energy_object()");

        let new_game_field_energy_map = GameFieldEnergy::new(1);
        self.game_field_energy_map.insert(account_unique_id, new_game_field_energy_map);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_create_field_energy_object() {
        let repository = FieldEnergyRepositoryImpl::new();
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
    async fn test_get_instance() {
        let instance1 = FieldEnergyRepositoryImpl::get_instance();
        let instance2 = FieldEnergyRepositoryImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }
}
