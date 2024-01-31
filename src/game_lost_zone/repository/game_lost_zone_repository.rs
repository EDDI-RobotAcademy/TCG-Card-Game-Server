pub trait GameLostZoneRepository {
    fn create_game_lost_zone_object(&mut self, account_unique_id: i32) -> bool;
}