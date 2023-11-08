use crate::node::Node;
use crate::puzzle::Puzzle;
use crate::r#move::Move;
use crate::solution::Solution;
use std::collections::HashMap;
use std::i64;

pub struct Solver {
    n_colors: usize,
    n_empty_vials: usize,
    n_volume: usize,
    n_vials: usize,
    puzzle: Puzzle,

    pub state: Vec<Vec<Vec<Node>>>,
    pub hashbits: HashMap<i32, i64>,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Self {
        const N_NOTDECREASE: usize = 1000;
        let n_colors = puzzle.n_colors;
        let n_empty_vials = puzzle.n_empty_vials;
        let n_volume = puzzle.n_volume;
        let mut state: Vec<Vec<Vec<Node>>> = Vec::new();

        let length_0 = n_colors * (n_volume - 1) + 1;
        let length_1 = N_NOTDECREASE + 1;

        for _ in 0..length_0 {
            let mut temp: Vec<Vec<Node>> = Vec::new();
            for _ in 0..length_1 {
                temp.push(Vec::new());
            }
            state.push(temp);
        }

        Solver {
            puzzle,
            state,
            n_colors,
            n_empty_vials,
            n_volume,
            n_vials: n_colors + n_empty_vials,
            hashbits: HashMap::new(),
        }
    }

    fn nearoptimal_solution_single(&self, n_block: usize, y0: usize) -> Option<Vec<Move>> {
        if self.state[n_block - self.n_colors][y0].is_empty() {
            return None;
        }
        let mut moves: Vec<Move> = Vec::new();

        if n_block == self.n_colors {
            let last_moves = self.state[0][0][0].last_moves(self.n_colors);
            moves.extend(last_moves);
            return Some(moves);
        }

        let mut x = n_block - self.n_colors;
        let mut y = y0;
        let mut nd = &self.state[x][y][0];
        let mv2 = nd.last_moves(self.n_colors);

        let mut src = nd.move_info.from;
        let mut dst = nd.move_info.to;

        moves.push(Move { from: src, to: dst });
        if nd.move_info.merged {
            x -= 1;
        } else {
            y -= 1;
        }

        while x != 0 || y != 0 {
            let nodes = &self.state[x][y];
            for test_node in nodes {
                let mut source_index = 0;
                while test_node[source_index].position != src {
                    source_index += 1;
                }

                let mut dest_index = 0;
                while test_node[dest_index].position != dst {
                    dest_index += 1;
                }

                let source_vial = test_node[source_index].get_vial_top_info();
                let dest_vial = test_node[dest_index].get_vial_top_info();

                if source_vial.empty == self.n_volume {
                    continue;
                }

                if dest_vial.empty == 0
                    || (dest_vial.empty < self.n_volume && source_vial.color != dest_vial.color)
                    || (dest_vial.empty == self.n_volume && source_vial.empty == self.n_volume - 1)
                {
                    continue;
                }

                let new_node = test_node.perform_move(
                    source_index,
                    dest_index,
                    source_vial.empty,
                    dest_vial.empty,
                    false,
                );

                let h = new_node.hash;
                let h2 = nd.hash;
                if h != h2 {
                    continue;
                }

                nd = test_node;
                src = nd.move_info.from;
                dst = nd.move_info.to;
                moves.push(Move { from: src, to: dst });
                if nd.move_info.merged {
                    x -= 1;
                } else {
                    y -= 1;
                }

                break;
            }
        }

        moves.reverse();
        moves.extend(mv2);
        Some(moves)
    }

    pub fn solve(&mut self) -> Solution {
        let mut nd = Node::new(&self.puzzle);
        nd.sort();

        let mut y = 0;
        let n_block_v = nd.node_blocks() + nd.empty_vials() - self.n_empty_vials;

        nd.write_hash(&mut self.hashbits);

        self.state[0][0].push(nd);

        let mut total = 1;
        let mut solution_found = false;

        loop {
            let mut new_nodes = 0;
            for x in 0..n_block_v - self.n_colors {
                for i in 0..self.state[x][y].len() {
                    for source_vial_index in 0..self.n_vials {
                        if self.state[x][y][i][source_vial_index].is_empty() {
                            continue;
                        }

                        let source_vial_top_info =
                            self.state[x][y][i][source_vial_index].get_vial_top_info();

                        for dest_vial_index in 0..self.n_vials {
                            if dest_vial_index == source_vial_index {
                                continue;
                            }

                            let dest_vial = &self.state[x][y][i][dest_vial_index];

                            if dest_vial.is_full() {
                                continue;
                            }

                            if dest_vial.is_empty()
                                && source_vial_top_info.empty == self.n_volume - 1
                            {
                                continue;
                            }

                            let dest_vial_top_info = dest_vial.get_vial_top_info();
                            if !dest_vial.is_empty()
                                && dest_vial_top_info.color != source_vial_top_info.color
                            {
                                continue;
                            }

                            let block_decrease_q = source_vial_top_info.full == 1
                                && source_vial_top_info.empty != self.n_volume - 1;
                            let new_node = self.state[x][y][i].perform_move(
                                source_vial_index,
                                dest_vial_index,
                                source_vial_top_info.empty,
                                dest_vial_top_info.empty,
                                block_decrease_q,
                            );

                            if new_node.is_hashed(&mut self.hashbits) {
                                continue;
                            }

                            total += 1;
                            new_node.write_hash(&mut self.hashbits);
                            if block_decrease_q {
                                self.state[x + 1][y].push(new_node);
                            } else {
                                self.state[x][y + 1].push(new_node);
                                new_nodes += 1;
                            }
                        }
                    }
                }
            }

            if !self.state[n_block_v - self.n_colors][y].is_empty() {
                solution_found = true;
            }
            y += 1;

            if solution_found || new_nodes == 0 {
                break;
            }
        }

        if solution_found {
            let mut lmin = 99999;
            let mut kmin = 0;
            for k in 0..self.state[n_block_v - self.n_colors][y - 1].len() {
                let j = self.state[n_block_v - self.n_colors][y - 1][k]
                    .n_last_moves(self.n_empty_vials);
                if j < lmin {
                    kmin = k;
                    lmin = j;
                }
            }

            self.state[n_block_v - self.n_colors][y - 1].swap(0, kmin);
        }

        let s = self.nearoptimal_solution_single(n_block_v, y - 1);

        if s.is_some() {
            return Solution {
                solved: true,
                moves: self.nearoptimal_solution_single(n_block_v, y - 1).unwrap(),
                nodes: total,
            };
        }

        Solution {
            solved: false,
            moves: Vec::new(),
            nodes: total,
        }
    }
}
