use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct GetSpecificRaceCardRequest {
    account_session_id: String,
    race_name: String,
    is_confirmed_upper_legend: bool,
}

impl GetSpecificRaceCardRequest {
    pub fn new(account_id: String, race_name: String, is_confirmed_upper_legend: bool) -> Self {
        GetSpecificRaceCardRequest { account_session_id: account_id.to_string(), race_name: race_name.to_string(), is_confirmed_upper_legend }
    }
    pub fn account_session_id(&self) -> &str { &self.account_session_id }
    pub fn get_race_enum(&self) -> RaceEnum {
        match self.race_name.as_str() {
            "Dummy" => RaceEnum::Dummy,
            "Undead" => RaceEnum::Undead,
            "Human" => RaceEnum::Human,
            "Trent" => RaceEnum::Trent,
            "Angel" => RaceEnum::Angel,
            "Machine" => RaceEnum::Machine,
            "Chaos" => RaceEnum::Chaos,
            _ => {
                eprintln!("Invalid race name: {}", self.race_name);
                RaceEnum::Dummy
            }
        }
    }

    pub fn is_confirmed_upper_legend(&self) -> bool { self.is_confirmed_upper_legend }
}