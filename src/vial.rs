use crate::constants::{BallType, MAX_VIAL_SIZE};
use crate::vial_top_info::VialTopInfo;
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vial {
    balls: [BallType; MAX_VIAL_SIZE],
    pub position: usize,
    pub depth: usize,
}

impl Vial {
    pub fn new(depth: usize, position: usize) -> Self {
        Vial {
            balls: [0; MAX_VIAL_SIZE],
            position,
            depth,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.balls[self.depth - 1] == 0
    }

    pub fn is_full(&self) -> bool {
        self.balls[0] != 0
    }

    pub fn get_vial_top_info(&self) -> VialTopInfo {
        let mut empty = 0;
        let mut full = 0;
        let mut color = self.balls[0];

        for ball in self {
            if *ball == 0 {
                empty += 1;
                continue;
            }

            if color == 0 {
                color = *ball;
                full += 1;
                continue;
            }

            if color == *ball {
                full += 1;
                continue;
            }

            break;
        }

        VialTopInfo { color, empty, full }
    }

    pub fn vial_blocks(&self) -> usize {
        let mut res = 1;

        for i in 0..self.depth - 1 {
            if self.balls[i + 1] != self.balls[i] {
                res += 1;
            }
        }

        if self.balls[0] == 0 {
            res -= 1;
        }

        res
    }
}

impl Index<usize> for Vial {
    type Output = BallType;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.depth);
        &self.balls[index]
    }
}

impl IndexMut<usize> for Vial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.depth);
        &mut self.balls[index]
    }
}

impl PartialOrd for Vial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vial {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.depth {
            if self[i] < other[i] {
                return Ordering::Less;
            }

            if self[i] > other[i] {
                return Ordering::Greater;
            }
        }
        
        Ordering::Equal
    }
}

impl PartialEq for Vial {
    fn eq(&self, other: &Self) -> bool {
        self.balls[..self.depth] == other.balls[..other.depth]
    }
}

impl Eq for Vial {
    
}

impl<'a> IntoIterator for &'a Vial {
    type Item = &'a BallType;
    type IntoIter = std::slice::Iter<'a, BallType>;

    fn into_iter(self) -> Self::IntoIter {
        self.balls[..self.depth].iter()
    }
}
