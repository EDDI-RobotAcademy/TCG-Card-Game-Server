use async_trait::async_trait;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

#[async_trait]
pub trait GameCardEnergyRepository {
    unsafe fn call_energy_card_repository_handler(&self, energy_card_id: i32) -> SummaryEnergyCardEffect;
}