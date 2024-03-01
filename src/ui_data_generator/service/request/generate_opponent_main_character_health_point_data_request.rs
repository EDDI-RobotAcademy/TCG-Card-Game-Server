#[derive(Debug)]
pub struct GenerateOpponentMainCharacterHealthPointDataRequest {
    main_character_updated_health_point: i32,
}

impl GenerateOpponentMainCharacterHealthPointDataRequest {
    pub fn new(main_character_updated_health_point: i32) -> Self {
        GenerateOpponentMainCharacterHealthPointDataRequest {
            main_character_updated_health_point
        }
    }

    pub fn get_main_character_updated_health_point(&self) -> i32 { self.main_character_updated_health_point }
}