use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_activation_energy::repository::card_activation_energy_repository::CardActivationEnergyRepository;

use crate::card_activation_energy::repository::card_activation_energy_repository_impl::CardActivationEnergyRepositoryImpl;
use crate::card_activation_energy::service::card_activation_energy_service::CardActivationEnergyService;
use crate::common::card_attributes::card_activation_energy::activation_energy_enum::ActivationEnergyEnum;

pub struct CardActivationEnergyServiceImpl {
    card_activation_energy_repository: Arc<AsyncMutex<CardActivationEnergyRepositoryImpl>>
}

impl CardActivationEnergyServiceImpl {
    pub fn new(card_activation_energy_repository: Arc<AsyncMutex<CardActivationEnergyRepositoryImpl>>) -> Self {
        CardActivationEnergyServiceImpl {
            card_activation_energy_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardActivationEnergyServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardActivationEnergyServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardActivationEnergyServiceImpl::new(
                            CardActivationEnergyRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardActivationEnergyService for CardActivationEnergyServiceImpl {
    async fn get_card_activation_energy(&self, card_number: &i32) -> i32 {
        println!("CardActivationEnergyServiceImpl: get_card_activation_energy()");

        let card_activation_energy_repository_guard = self.card_activation_energy_repository.lock().await;
        card_activation_energy_repository_guard.get_card_activation_energy(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_activation_energy() {
        let card_activation_energy_repo_mutex = CardActivationEnergyRepositoryImpl::get_instance();
        let card_activation_energy_guard = card_activation_energy_repo_mutex.lock().await;
        let card_number: i32 = 19;
        let card_activation_energy = card_activation_energy_guard.get_card_activation_energy(&card_number).await;


        println!("Card Activation Energy: {:?}", card_activation_energy);

    }
}