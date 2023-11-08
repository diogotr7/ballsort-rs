mod constants;
mod r#move;
mod move_info;
mod node;
mod puzzle;
mod solution;
mod solver;
mod vial;
mod vial_top_info;

use crate::puzzle::Puzzle;
use solver::Solver;

fn main() {
    let puzzle = Puzzle::new_parse("big.txt");

    let mut solver = Solver::new(&puzzle);

    let now = std::time::Instant::now();
    let solution = solver.solve();
    let elapsed = now.elapsed();

    println!("Solution: {:?}", solution);
    println!("Elapsed: {:?}", elapsed);
}
