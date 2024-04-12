use crate::board::Board;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};

pub fn solve(board: Board) -> Option<Board> {
    let between = Uniform::from(1..=9);
    let mut rng = rand::thread_rng();
    loop {
        let mut board = board.clone();
        for (x, y) in (0..9).cartesian_product(0..9) {
            if board.get(x, y).is_none() {
                let val = between.sample(&mut rng);
                board.set(x, y, Some(val))
            }
        }
        if board.is_valid() && board.is_filled() {
            return Some(board);
        }
    }
}
