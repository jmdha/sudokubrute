mod backtracking;

use crate::board::Board;

pub enum SolveKind {
    Backtracking,
}

pub fn solve(kind: SolveKind, board: Board) -> Option<Board> {
    match kind {
        SolveKind::Backtracking => backtracking::solve(board),
    }
}
