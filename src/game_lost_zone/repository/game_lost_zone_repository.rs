pub trait GameLostZoneRepository {
    fn create_game_lost_zone_object(&mut self, account_unique_id: i32) -> bool;
    fn add_lost_zone_card(&mut self, account_unique_id: i32, lost_card: i32) -> bool;
}