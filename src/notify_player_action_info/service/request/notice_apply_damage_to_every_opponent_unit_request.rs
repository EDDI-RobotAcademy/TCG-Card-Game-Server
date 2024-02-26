#[derive(Debug)]
pub struct NoticeApplyDamageToEveryOpponentUnitRequest {
    opponent_unique_id: i32,
    damage: i32,
    updated_health_point_list: Vec<i32>,
    dead_unit_index_list: Vec<i32>,
}

impl NoticeApplyDamageToEveryOpponentUnitRequest {
    pub fn new(opponent_unique_id: i32,
               damage: i32,
               updated_health_point_list: Vec<i32>,
               dead_unit_index_list: Vec<i32>,) -> Self {
        NoticeApplyDamageToEveryOpponentUnitRequest {
            opponent_unique_id,
            damage,
            updated_health_point_list,
            dead_unit_index_list
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_damage(&self) -> i32 { self.damage }

    pub fn get_updated_health_point_list(&self) -> &Vec<i32> { &self.updated_health_point_list }

    pub fn get_dead_unit_index_list(&self) -> &Vec<i32> { &self.dead_unit_index_list }
}