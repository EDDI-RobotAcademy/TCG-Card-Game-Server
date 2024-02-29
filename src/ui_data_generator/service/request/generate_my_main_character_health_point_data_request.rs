#[derive(Debug)]
pub struct GenerateMyMainCharacterHealthPointDataRequest {
    main_character_index:i32,
    main_character_updated_health_point: i32,
}

impl GenerateMyMainCharacterHealthPointDataRequest {
    pub fn new(main_character_index:i32,main_character_updated_health_point: i32) -> Self {
        GenerateMyMainCharacterHealthPointDataRequest {
            main_character_index,
            main_character_updated_health_point
        }
    }

    pub fn get_main_character_index(&self) -> i32 { self.main_character_index }
    pub fn get_main_character_updated_health_point(&self) -> i32 { self.main_character_updated_health_point }
}