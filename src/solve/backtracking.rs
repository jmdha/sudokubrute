use crate::board::Board;

pub fn solve(board: Board) -> Option<Board> {
    _solve(board.clone(), 0, 81 - board.count())
}

fn _solve(board: Board, i: usize, missing: usize) -> Option<Board> {
    if missing == 0 {
        return Some(board);
    }
    let cell = board.get_i(i);
    if cell != 0 {
        return _solve(board, i + 1, missing);
    }
    for v in 1..=9 {
        if let Some(board) = board.set_i(i, v) {
            if let Some(board) = _solve(board, i + 1, missing - 1) {
                return Some(board);
            }
        }
    }
    None
}
