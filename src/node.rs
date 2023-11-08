use crate::constants::{BallType, MAX_VIALS};
use crate::move_info::MoveInfo;
use crate::puzzle::Puzzle;
use crate::r#move::Move;
use crate::vial::Vial;
use std::collections::HashMap;
use std::i64;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Node {
    vials: [Vial; MAX_VIALS],
    pub length: usize,
    pub move_info: MoveInfo,
    pub hash: i32,
}

impl Index<usize> for Node {
    type Output = Vial;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.length);
        &self.vials[index]
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.length);
        &mut self.vials[index]
    }
}

impl Node {
    pub fn new(puzzle: &Puzzle) -> Self {
        let vial_count = puzzle.vials.len();
        let vial_depth = puzzle.vials[0].len();

        let mut vials: [Vial; MAX_VIALS] = std::array::from_fn(|position| {
            let pos = if position < vial_count { position } else { 0 };
            Vial::new(vial_depth, pos)
        });

        for i in 0..puzzle.vials.len() {
            for j in 0..puzzle.vials[0].len() {
                vials[i][j] = puzzle.vials[i][j];
            }
        }

        let mut node = Node {
            vials,
            hash: 0,
            length: vial_count,
            move_info: MoveInfo {
                from: 0,
                to: 0,
                merged: false,
            },
        };

        node.hash = node.hash();

        node
    }

    pub fn perform_move(
        &self,
        source_vial_index: usize,
        target_vial_index: usize,
        source_empty_count: usize,
        target_empty_count: usize,
        flag: bool,
    ) -> Node {
        //copy the data we already have before
        let mut vials: [Vial; MAX_VIALS] = self.vials.clone();

        let temp = vials[source_vial_index][source_empty_count];
        vials[target_vial_index][target_empty_count - 1] = temp;
        vials[source_vial_index][source_empty_count] = 0;

        let mut node = Node {
            vials,
            hash: 0,
            length: self.length,
            move_info: MoveInfo {
                from: self.vials[source_vial_index].position,
                to: self.vials[target_vial_index].position,
                merged: flag,
            },
        };

        node.sort();
        
        node.hash = node.hash();

        node
    }

    pub fn write_hash(&self, dict: &mut HashMap<i32, i64>) {
        let base = self.hash / 64;
        let offset = self.hash % 64;
        let one: i64 = 1;
        let magic = offset & 0x1f;
        let x = one << magic;

        let y = dict.entry(base).or_insert(0);

        *y |= x;
    }

    pub fn is_hashed(&self, dict: &mut HashMap<i32, i64>) -> bool {
        let base = self.hash / 64;
        let offset = self.hash % 64;
        let one: i64 = 1;
        let magic = offset & 0x1f;
        let x = one << magic;
        let y = dict.entry(base).or_insert(0);

        *y & x != 0
    }

    pub fn last_moves(&self, n_colors: usize) -> Vec<Move> {
        let mut moves = Vec::new();

        for i in 1..n_colors {
            let color = i as BallType;
            let mut j = self.length - 1;
            while self.vials[j].get_vial_top_info().color != color {
                j -= 1;
            }

            //TODO: replace this with is_empty???
            if self.vials[j].get_vial_top_info().empty == 0 {
                continue;
            }

            for k in 0..j {
                let top_info = self.vials[k].get_vial_top_info();
                if top_info.color == color {
                    for _ in 0..top_info.full {
                        moves.push(Move {
                            from: self.vials[k].position,
                            to: self.vials[j].position,
                        })
                    }
                }
            }
        }

        moves
    }

    pub fn n_last_moves(&self, n_empty_vials: usize) -> usize {
        let mut r = 0;

        for i in 0..n_empty_vials {
            r += self.vials[i].get_vial_top_info().full;
        }

        r
    }

    pub fn node_blocks(&self) -> usize {
        let mut r = 0;

        for vial in self {
            r += vial.vial_blocks();
        }

        r
    }

    pub fn empty_vials(&self) -> usize {
        let mut r = 0;

        for vial in self {
            if vial.is_empty() {
                r += 1;
            }
        }

        r
    }

    pub fn sort(&mut self) {
        self.vials[..self.length].sort();
    }

    fn hash(&self) -> i32 {
        let mut hash: i32 = 0;

        for vial in self {
            for ball in vial {
                hash = hash.overflowing_mul(31).0.overflowing_add(*ball as i32).0;
            }
        }

        hash
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = &'a Vial;
    type IntoIter = std::slice::Iter<'a, Vial>;

    fn into_iter(self) -> Self::IntoIter {
        self.vials[..self.length].iter()
    }
}
