use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::entity::harmful_status_effect::HarmfulStatusEffect;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::entity::unit_health_point::UnitHealthPoint;

#[derive(Debug, Clone)]
pub struct GameFieldUnitCard {
    field_unit_card: i32,
    attached_energy_map: AttachedEnergyMap,
    field_unit_race: RaceEnum,
    field_unit_grade: GradeEnum,
    unit_attack_point: i32,
    unit_health_point: UnitHealthPoint,
    unit_attack_required_energy: i32,
    has_first_passive_skill: bool,
    has_second_passive_skill: bool,
    has_third_passive_skill: bool,
    extra_status_effect_list: Vec<ExtraStatusEffect>,
    harmful_status_effect_list: Vec<HarmfulStatusEffect>,
    turn_action: bool,
    is_alive: bool,
    deployed_round: i32,
    passive_status_list: Vec<PassiveStatus>
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32,
               field_unit_race: RaceEnum,
               field_unit_grade: GradeEnum,
               unit_attack_point: i32,
               unit_health_point: i32,
               unit_attack_required_energy: i32,
               has_first_passive_skill: bool,
               has_second_passive_skill: bool,
               has_third_passive_skill: bool,
               is_alive: bool) -> GameFieldUnitCard {


        GameFieldUnitCard {
            field_unit_card,
            attached_energy_map: AttachedEnergyMap::new(),
            field_unit_race,
            field_unit_grade,
            unit_attack_point,
            unit_health_point: UnitHealthPoint::new(unit_health_point),
            unit_attack_required_energy,
            has_first_passive_skill,
            has_second_passive_skill,
            has_third_passive_skill,
            extra_status_effect_list: Vec::new(),
            harmful_status_effect_list: Vec::new(),
            is_alive,
            turn_action: false,
            deployed_round: -1,
            passive_status_list: Vec::new()
        }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }

    pub fn get_attached_energy(&self) -> &AttachedEnergyMap {
        &self.attached_energy_map
    }

    pub fn get_unit_health_point(&self) -> &UnitHealthPoint {
        &self.unit_health_point
    }

    pub fn get_mut_unit_health_point(&mut self) -> &mut UnitHealthPoint {
        &mut self.unit_health_point
    }

    pub fn get_unit_attack_point(&self) -> i32 {
        self.unit_attack_point
    }

    pub fn get_extra_status_effect_list(&self) -> &Vec<ExtraStatusEffect> {
        &self.extra_status_effect_list
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_turn_action(&self) -> bool {
        self.turn_action
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }

    pub fn set_turn_action(&mut self, turn_action: bool) {
        self.turn_action = turn_action;
    }

    pub fn attach_energy(&mut self, race: RaceEnumValue, quantity: i32) {
        self.attached_energy_map.add_energy(race, quantity);
    }

    pub fn detach_energy(&mut self, race_enum: RaceEnumValue, quantity: i32) {
        self.attached_energy_map.remove_energy(&race_enum, quantity);
    }

    pub fn increase_max_health(&mut self, increase_point: i32) {
        self.unit_health_point.increase_max_health(increase_point);
    }

    pub fn apply_damage(&mut self, damage: i32) {
        let remaining_health = self.unit_health_point.get_current_health_point() - damage;
        let current_health = remaining_health.max(0);

        self.unit_health_point.set_current_health_point(current_health);
    }

    // TODO: 이 부분도 Domain 이 점점 커지고 있음 (Deadline 고려하면 현재는 수습 불가)
    // 네이밍 이슈로 harmful_status_effect 를 별개로 구성 (해로운 효과와 에너지 부착으로 추가 획득한 효과가 구별되어야함)
    pub fn attach_special_energy(&mut self, race: RaceEnumValue, quantity: i32, status_effect_list: Vec<StatusEffect>) {
        self.attached_energy_map.add_energy(race, quantity);
        for status_effect in status_effect_list.iter().cloned() {
            let effect_number = status_effect.get_effect().to_i32();
            self.extra_status_effect_list.push(ExtraStatusEffect::new(
                ExtraEffect::from(effect_number),
                status_effect.get_status_duration_turn(),
                status_effect.get_effect_damage(),
                status_effect.get_reuse_turn()));
        }
    }

    // 공격을 통한 상태 이상 부여
    pub fn impose_harmful_state(&mut self, harmful_state: ExtraStatusEffect) {
        let harmful_status_effect = HarmfulStatusEffect::new(
            harmful_state.get_extra_effect().clone(),
            harmful_state.get_status_duration_turn(),
            harmful_state.get_effect_damage(),
            harmful_state.get_reuse_turn(),
        );

        self.harmful_status_effect_list.push(harmful_status_effect);
    }

    pub fn impose_harmful_state_list(&mut self, harmful_states: Vec<ExtraStatusEffect>) {
        for harmful_state in harmful_states {
            self.impose_harmful_state(harmful_state);
        }
    }

    // 상태 이상에 따른 데미지 및 효과 처리
    pub fn apply_status_effect_damage(&mut self) {
        let mut index_to_remove = Vec::new();

        for index in (0..self.harmful_status_effect_list.len()).rev() {
            self.apply_damage_from_effect(index);
            self.decrease_status_duration(index);
            self.decrease_reuse_turn(index);

            // 만약 상태 지속 턴이 0이 되었을 경우 해당 상태 효과를 제거
            if self.harmful_status_effect_list[index].get_status_duration_round() == 0 {
                index_to_remove.push(index);
            }
        }

        // index_to_remove 에 저장된 인덱스들을 제거
        for index in index_to_remove {
            self.harmful_status_effect_list.remove(index);
        }
    }

    // ExtraStatusEffect 의 효과 데미지 적용
    fn apply_damage_from_effect(&mut self, index: usize) {
        let effect_damage = self.harmful_status_effect_list[index].get_effect_damage();
        if effect_damage > 0 {
            let current_health = self.unit_health_point.get_current_health_point();
            let new_health = current_health.saturating_sub(effect_damage);
            self.unit_health_point.set_current_health_point(new_health);
        }
    }

    // ExtraStatusEffect 의 상태 지속 턴 감소
    fn decrease_status_duration(&mut self, index: usize) {
        let current_duration = self.harmful_status_effect_list[index].get_status_duration_round();
        if current_duration > 0 {
            self.harmful_status_effect_list[index].set_status_duration_round(current_duration - 1);
        }
    }

    // ExtraStatusEffect 의 재사용 턴 감소 (빙결의 경우 같은 유닛을 계속 얼릴 수 없음)
    fn decrease_reuse_turn(&mut self, index: usize) {
        let current_reuse_turn = self.harmful_status_effect_list[index].get_reapply_round();
        if current_reuse_turn > 0 {
            self.harmful_status_effect_list[index].set_reapply_round(current_reuse_turn - 1);
        }
    }

    pub fn impose_extra_effect_state(&mut self, extra_effect_state: ExtraStatusEffect) {
        self.extra_status_effect_list.push(extra_effect_state);
    }

    fn impose_extra_effect_state_list(&mut self, extra_effect_state_list: Vec<ExtraStatusEffect>) {
        for extra_effect_state in extra_effect_state_list {
            self.impose_extra_effect_state(extra_effect_state);
        }
    }

    pub fn set_deployed_round(&mut self, round: i32) {
        self.deployed_round = round;
    }

    pub fn get_deployed_round(&self) -> i32 {
        self.deployed_round
    }

    pub fn set_passive_status_list(&mut self, passive_status_list: Vec<PassiveStatus>) {
        self.passive_status_list = passive_status_list;
    }

    pub fn get_passive_status_list(&self) -> &Vec<PassiveStatus> {
        &self.passive_status_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_field_unit_card_creation() {
        let game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        assert_eq!(game_field_unit_card.get_card(), 5);
        println!("Test passed: FieldUnit creation and getter");
    }

    #[test]
    fn test_attach_energy() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        assert_eq!(game_field_unit_card.get_card(), 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 0);

        game_field_unit_card.attach_energy(RaceEnumValue::Undead, 3);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 1);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Undead).unwrap(), 3);

        game_field_unit_card.attach_energy(RaceEnumValue::Human, 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 2);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Human).unwrap(), 5);

        println!("{:?}", game_field_unit_card);

        println!("Test passed: FieldUnit creation, getter, attach_energy, and print_state");
    }

    #[test]
    fn test_increase_max_health() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        println!("game_field_unit_card: {:?}", game_field_unit_card);

        game_field_unit_card.increase_max_health(10);
        println!("game_field_unit_card: {:?}", game_field_unit_card);

        assert_eq!(game_field_unit_card.get_unit_health_point().get_max_health_point(), 30);
        assert_eq!(game_field_unit_card.get_unit_health_point().get_current_health_point(), 30);

        println!("Test passed: increase_max_health");
    }

    #[test]
    fn test_apply_damage() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        println!("unit: {:?}", game_field_unit_card);
        game_field_unit_card.apply_damage(5);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            15
        );

        println!("unit: {:?}", game_field_unit_card);
        game_field_unit_card.apply_damage(20);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            0
        );

        println!("unit: {:?}", game_field_unit_card);
        println!("Test passed: apply_damage");
    }

    #[test]
    fn test_apply_status_effect_damage() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        game_field_unit_card.extra_status_effect_list.push(ExtraStatusEffect::new(
            ExtraEffect::Freeze,
            2,
            5,
            3,
        ));

        println!("Before apply_status_effect_damage: {:?}", game_field_unit_card);
        game_field_unit_card.apply_status_effect_damage();
        println!("Test result: {:?}", game_field_unit_card);

        game_field_unit_card.apply_status_effect_damage();
        println!("Test result: {:?}", game_field_unit_card);

        game_field_unit_card.apply_status_effect_damage();
        println!("Test result: {:?}", game_field_unit_card);
    }

    #[test]
    fn test_impose_harmful_state_and_apply_damage() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        let harmful_state = ExtraStatusEffect::new(ExtraEffect::DarkFire, 5, 5, 0);
        game_field_unit_card.impose_harmful_state(harmful_state.clone());

        println!("Before apply_status_effect_damage: {:?}", game_field_unit_card);
        game_field_unit_card.apply_status_effect_damage();
        println!("Test result after 1st apply_status_effect_damage: {:?}", game_field_unit_card);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            15
        );

        // TODO: 같은 상태 이상에 대해서 적용되는 duration 값만 초기화하도록 구성이 필요함
        // game_field_unit_card.impose_harmful_state(harmful_state.clone());
        game_field_unit_card.apply_status_effect_damage();

        println!("Test result after 2nd apply_status_effect_damage: {:?}", game_field_unit_card);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            10
        );

        game_field_unit_card.apply_status_effect_damage();

        println!("Test result after 3rd apply_status_effect_damage: {:?}", game_field_unit_card);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            5
        );
    }
}
