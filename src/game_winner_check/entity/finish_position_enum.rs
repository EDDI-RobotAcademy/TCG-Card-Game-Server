#[derive(Debug, Hash, Eq, PartialEq)]
pub enum FinishPositionEnum {
    Dummy = 0,
    Winner = 1,
    Loser = 2,
    Draw = 3,
}