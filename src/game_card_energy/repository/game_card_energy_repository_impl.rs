use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::effect::Effect;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

use crate::game_card_energy::handler::game_card_energy_handler::GameCardEnergyHandler;
use crate::game_card_energy::handler::handler_of_93::game_card_energy_93_handler_impl::EnergyCard_93_Function;
use crate::game_card_energy::repository::game_card_energy_repository::GameCardEnergyRepository;

pub struct GameCardEnergyRepositoryImpl {
    energy_card_functions: HashMap<i32, Box<dyn GameCardEnergyHandler>>,
}

struct NoneEnergyFunction;

impl GameCardEnergyHandler for NoneEnergyFunction {
    unsafe fn use_specific_energy_card(&self) -> SummaryEnergyCardEffect {
        println!("아직 구현되지 않은 기능입니다.");

        SummaryEnergyCardEffect::new(
            RaceEnum::Dummy, vec![StatusEffect::new(Effect::Dummy, -1, -1, -1)])
    }
}

impl GameCardEnergyRepositoryImpl {
    fn new() -> Self {
        let mut energy_card_functions = HashMap::new();
        energy_card_functions.insert(93, Box::new(EnergyCard_93_Function) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(97, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(99, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(110, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(111, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(118, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);
        // energy_card_functions.insert(151, Box::new(EnergyCard_151_Function) as Box<dyn GameCardEnergyHandler>);
        energy_card_functions.insert(185, Box::new(NoneEnergyFunction) as Box<dyn GameCardEnergyHandler>);

        GameCardEnergyRepositoryImpl { energy_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardEnergyHandler>> {
        self.energy_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardEnergyRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardEnergyRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardEnergyRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardEnergyRepository for GameCardEnergyRepositoryImpl {
    unsafe fn call_energy_card_repository_handler(&self, energy_card_id: i32) -> SummaryEnergyCardEffect {
        println!("GameCardEnergyRepositoryImpl: call_energy_card_repository_handler()");

        let energy_card_summary_handler = self.energy_card_functions.get(&energy_card_id);
        energy_card_summary_handler.unwrap().use_specific_energy_card()
    }
}