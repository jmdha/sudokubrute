use sudokubrute::{board::Board, solve::solve};

fn main() {
    let board = Board::from(
        "004300209005009001070060043006002087190007400050083000600000105003508690042910300",
    );
    loop {
        solve(sudokubrute::solve::SolveKind::Backtracking, board.clone());
    }
}
