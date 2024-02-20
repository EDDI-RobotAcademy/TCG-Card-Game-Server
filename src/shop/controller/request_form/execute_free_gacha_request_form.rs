use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::redis::service::request::save_daily_key_and_value_request::SaveDailyKeyAndValueRequest;
use crate::shop_gacha::service::request::get_specific_race_card_request::GetSpecificRaceCardRequest;

#[derive(Debug)]
pub struct ExecuteFreeGachaRequestForm {
    account_session_id: String,
    race_name: String,
    is_confirmed_upper_legend: bool,
}

impl ExecuteFreeGachaRequestForm {
    pub fn new(account_session_id: String, race_name: String, is_confirmed_upper_legend: bool) -> Self {
        ExecuteFreeGachaRequestForm { account_session_id: account_session_id.to_string(), race_name: race_name.to_string(), is_confirmed_upper_legend }
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
    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.account_session_id.clone().as_str())
    }
    pub fn to_save_daily_key_and_value_request(&self, account_id: &str) -> SaveDailyKeyAndValueRequest {
        SaveDailyKeyAndValueRequest::new(self.account_session_id.clone().as_str(), account_id)
    }
    pub fn to_get_specific_race_card_request(&self, account_unique_id: i32, race_name: RaceEnum, is_confirmed_upper_legend: bool) -> GetSpecificRaceCardRequest {
        GetSpecificRaceCardRequest::new(account_unique_id, race_name, is_confirmed_upper_legend)
    }
}