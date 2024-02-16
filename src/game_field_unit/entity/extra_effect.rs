#[derive(Clone, Debug, PartialEq)]

// TODO: 빙결 = 연속으로 적용 불가, 즉 빙결 상태에 있는지 여부를 확인해야 함
// TODO: 암흑 화염 = 2라운드 동안 적용, 데미지는 중첩되지 않으며 duration 만 바뀜
pub enum ExtraEffect {
    Dummy,
    Freeze = 1,
    Darkfire = 2,
    PhysicalImmunity = 3,
}

impl From<i32> for ExtraEffect {
    fn from(value: i32) -> Self {
        match value {
            1 => ExtraEffect::Freeze,
            2 => ExtraEffect::Darkfire,
            3 => ExtraEffect::PhysicalImmunity,
            _ => ExtraEffect::Dummy,
        }
    }
}
