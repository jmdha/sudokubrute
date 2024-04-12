use crate::board::Board;

pub fn solve(board: Board) -> Option<Board> {
    _solve(board, 0)
}

fn _solve(board: Board, i: usize) -> Option<Board> {
    if board.is_filled() {
        return Some(board);
    }
    let cell = board.get_i(i);
    if cell.is_some() {
        return _solve(board, i + 1);
    }
    for v in 1..=9 {
        let mut board = board.clone();
        board.set_i(i, Some(v));
        if !board.is_valid() {
            continue;
        }
        if let Some(board) = _solve(board, i + 1) {
            return Some(board);
        }
    }
    None
}
