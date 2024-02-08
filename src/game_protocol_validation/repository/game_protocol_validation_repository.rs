pub trait GameProtocolValidationRepository {
    fn record_cheating_player(&mut self, account_unique_id: i32, support_card_id: i32, current_round: i32) -> bool;
}