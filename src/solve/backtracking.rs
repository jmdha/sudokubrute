use crate::board::Board;

pub fn solve(mut board: Board) -> Option<Board> {
    let count = board.count();
    _solve(&mut board, 0, 81 - count)
}

fn _solve(board: &mut Board, i: usize, missing: usize) -> Option<Board> {
    if missing == 0 {
        return Some(board.clone());
    }
    if board.get_i(i) != 0 {
        return _solve(board, i + 1, missing);
    }
    for v in 1..=9 {
        let success = board.set_i(i, v);
        if success {
            if let Some(board) = _solve(board, i + 1, missing - 1) {
                return Some(board);
            }
            board.clear_i(i);
        }
    }
    None
}
