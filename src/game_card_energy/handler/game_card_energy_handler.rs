use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

pub trait GameCardEnergyHandler: Send {
    unsafe fn use_specific_energy_card(&self) -> SummaryEnergyCardEffect;
}