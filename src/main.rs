use std::time;

use sudokubrute::{board::Board, solve::solve};

fn main() {
    let board = Board::generate(
        "9..8...........5............2..1...3.1.....6....4...7.7.86.........3.1..4.....2..",
    )
    .unwrap();
    let t = time::Instant::now();
    let solved = solve(sudokubrute::solve::SolveKind::Backtracking, board.clone());
    if solved.is_none() {
        println!("unsolvable");
    }
    let board = solved.unwrap();
    let elapsed = t.elapsed().as_secs_f64();
    println!("Time taken: {}s", elapsed);
    println!("Times board was modified: {}", board.times_set());
    println!(
        "Modifications per second: {}",
        board.times_set() as f64 / elapsed
    );
    print!("{}", board);
}
