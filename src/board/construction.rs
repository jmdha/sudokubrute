use super::Board;

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let mut board = Board::default();
        value.as_bytes().into_iter().enumerate().for_each(|(i, c)| {
            let cell = match c.is_ascii_digit() {
                true => {
                    let num = (*c as char).to_digit(10).unwrap() as u8;
                    match num {
                        0 => None,
                        _ => Some(num),
                    }
                }
                false => None,
            };
            board.set_i(i, cell);
        });
        board
    }
}
