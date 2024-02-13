use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_activation_energy::activation_energy_enum::ActivationEnergyEnum;
// use crate::common::card_attributes::card_activation_energy::activation_energy_enum::ActivationEnergyEnum::ActivationEnergy;


#[async_trait]
pub trait CardActivationEnergyRepository {
    async fn get_card_activation_energy(&self, card_number: &i32) -> i32;

}




