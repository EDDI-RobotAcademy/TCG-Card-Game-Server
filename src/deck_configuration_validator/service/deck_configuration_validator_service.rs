use async_trait::async_trait;

#[async_trait]
pub trait DeckConfigurationValidatorService {
    async fn validate_deck(&self, deck: &Vec<i32>) -> Result<(), String>;
    async fn do_you_have_this_card(&self, card_list: Vec<i32>, account_unique_id: i32) -> bool ;
}