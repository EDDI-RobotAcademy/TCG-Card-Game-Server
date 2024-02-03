pub trait GameCardEnergyCountRepository {
    fn create_player_game_card_energy_count_object(&mut self, account_unique_id: i32) -> bool;
    fn add_energy_to_player_game_card(&mut self, account_unique_id: i32, card: i32) -> bool;
}