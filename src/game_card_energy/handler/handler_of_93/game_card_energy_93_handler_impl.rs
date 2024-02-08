use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_energy::handler::game_card_energy_handler::GameCardEnergyHandler;

pub struct EnergyCard_93_Function;

impl GameCardEnergyHandler for EnergyCard_93_Function {
    unsafe fn use_specific_energy_card(&self) -> SummaryEnergyCardEffect {
        println!("SupportCard_2_Function: use_specific_support_card()");

        SummaryEnergyCardEffect::new(RaceEnum::Undead)
    }
}