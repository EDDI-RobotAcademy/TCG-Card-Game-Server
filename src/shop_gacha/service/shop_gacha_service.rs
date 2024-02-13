use async_trait::async_trait;
use crate::shop_gacha::service::request::get_specific_race_card_request::GetSpecificRaceCardRequest;
use crate::shop_gacha::service::response::get_specific_race_card_response::GetSpecificRaceCardResponse;

#[async_trait]
pub trait ShopGachaService {
    async fn get_specific_race_card_default(&self, get_card_default_request: GetSpecificRaceCardRequest) -> GetSpecificRaceCardResponse ;

}