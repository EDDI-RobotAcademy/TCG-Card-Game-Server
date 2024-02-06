#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KindsEnum {
    Unit = 1,
    Item = 2,
    Trap = 3,
    Support = 4,
    Tool = 5,
    Energy = 6,
    Environment = 7,
    Token = 8,
}