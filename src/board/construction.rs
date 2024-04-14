use super::Board;

impl Board {
    pub fn generate(input: &str) -> Option<Board> {
        let mut board = Board::default();
        for (i, c) in input.as_bytes().into_iter().enumerate() {
            let cell = match c.is_ascii_digit() {
                true => (*c as char).to_digit(10).unwrap() as u8,
                false => 0,
            };
            if cell == 0 {
                continue;
            }
            if board.candidates_i(i) & 1 << cell == 0 {
                return None;
            }
            board.set_unchecked_i(i, cell);
        }
        Some(board)
    }
}
