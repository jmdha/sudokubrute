mod backtracking;
mod bogo;

use crate::board::Board;

pub enum SolveKind {
    Backtracking,
    Bogo,
}

pub fn solve(kind: SolveKind, board: Board) -> Option<Board> {
    match kind {
        SolveKind::Backtracking => backtracking::solve(board),
        SolveKind::Bogo => bogo::solve(board),
    }
}
