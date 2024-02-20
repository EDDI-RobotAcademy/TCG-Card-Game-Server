use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct GetSpecificRaceCardRequest {
    account_id: i32,
    race_name: RaceEnum,
    is_confirmed_upper_legend: bool,
}

impl GetSpecificRaceCardRequest {
    pub fn new(account_id: i32, race_name: RaceEnum, is_confirmed_upper_legend: bool) -> Self {
        GetSpecificRaceCardRequest { account_id: account_id, race_name: race_name, is_confirmed_upper_legend }
    }
    pub fn account_id(&self) -> i32 { self.account_id }
    pub fn get_race_enum(&self) -> RaceEnum { self.race_name }

    pub fn is_confirmed_upper_legend(&self) -> bool { self.is_confirmed_upper_legend }
}