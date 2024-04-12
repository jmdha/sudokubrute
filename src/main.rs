use sudokubrute::{board::Board, solve::solve};

fn main() {
    let board = Board::generate(
        "9..8...........5............2..1...3.1.....6....4...7.7.86.........3.1..4.....2..",
    )
    .unwrap();
    let solved = solve(sudokubrute::solve::SolveKind::Backtracking, board.clone());
    if solved.is_none() {
        println!("unsolvable");
    }
    print!("{}", solved.unwrap());
}
