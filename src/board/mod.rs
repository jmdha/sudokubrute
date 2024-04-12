pub mod construction;
pub mod debug;
pub mod validity;

#[derive(Clone, Default)]
pub struct Board {
    cells: [[u8; 9]; 9],
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.cells[x][y]
    }

    pub fn get_i(&self, i: usize) -> u8 {
        let x = i % 9;
        let y = (i - x) / 9;
        self.get(x, y)
    }

    pub fn set(&self, x: usize, y: usize, val: u8) -> Option<Board> {
        assert!(val != 0);
        if !self.is_valid(x, y, val) {
            return None;
        }
        let mut board = self.clone();
        board.cells[x][y] = val;
        Some(board)
    }

    pub fn set_i(&self, i: usize, val: u8) -> Option<Board> {
        let x = i % 9;
        let y = (i - x) / 9;
        self.set(x, y, val)
    }

    pub fn count(&self) -> usize {
        (0..81).filter(|i| self.get_i(*i) != 0).count()
    }

    pub fn is_filled(&self) -> bool {
        (0..81).all(|i| self.get_i(i) != 0)
    }
}
