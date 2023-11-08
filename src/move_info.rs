#[derive(Debug, PartialEq)]
pub struct MoveInfo {
    pub from: usize,
    pub to: usize,
    pub merged: bool,
}
