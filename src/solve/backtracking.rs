use crate::board::Board;

pub fn solve(mut board: Board) -> Option<Board> {
    let count = board.count();
    _solve(&mut board, 0, 81 - count)
}

fn _solve(board: &mut Board, i: usize, missing: usize) -> Option<Board> {
    debug_assert!(missing <= 81);
    if missing == 0 {
        return Some(board.clone());
    }
    debug_assert!(i < 81);
    if board.get_i(i) != 0 {
        return _solve(board, i + 1, missing);
    }
    let mut candidates = board.candidates_i(i);
    while candidates != 0 {
        let v = candidates.trailing_zeros();
        candidates &= !(1 << v);
        board.set_unchecked_i(i, v as u8);
        if let Some(board) = _solve(board, i + 1, missing - 1) {
            return Some(board);
        }
        board.clear_i(i);
    }
    None
}
