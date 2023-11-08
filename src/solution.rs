use crate::r#move::Move;

#[derive(Debug)]
pub struct Solution {
    pub solved: bool,
    pub moves: Vec<Move>,
    pub nodes: usize,
}
