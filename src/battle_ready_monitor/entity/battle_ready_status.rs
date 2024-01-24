#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BattleReadyStatus {
    WAIT,
    SUCCESS,
    FAIL,
}