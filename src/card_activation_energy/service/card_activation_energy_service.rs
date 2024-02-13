use async_trait::async_trait;

use crate::common::card_attributes::card_activation_energy::activation_energy_enum::ActivationEnergyEnum;


#[async_trait]
pub trait CardActivationEnergyService {
    async fn get_card_activation_energy(&self, card_number: &i32) -> i32;
}