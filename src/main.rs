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
    
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // 
    // loop {
    //     Solver::new(Puzzle::new_rand(12,2,4)).solve();
    // }
    
    let mut solver = Solver::new(Puzzle::new_parse("big.txt"));
    
    let now = std::time::Instant::now();
    let solution = solver.solve();
    let elapsed = now.elapsed();
    
    println!("Nodes: {}", solution.nodes);
    println!("Moves: {}", solution.moves.len());
    println!("Elapsed: {:?}", elapsed);
}
