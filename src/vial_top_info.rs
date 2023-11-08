use crate::constants::BallType;

#[derive(Debug)]
pub struct VialTopInfo {
    pub color: BallType,
    pub empty: usize,
    pub full: usize,
}
