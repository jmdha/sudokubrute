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
        debug_assert!(i < 81);
        let x = i % 9;
        let y = (i - x) / 9;
        self.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) -> bool {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        debug_assert_ne!(val, 0);
        if !self.is_valid(x, y, val) {
            return false;
        }
        self.cells[x][y] = val;
        true
    }

    pub fn set_i(&mut self, i: usize, val: u8) -> bool {
        debug_assert!(i < 81);
        debug_assert_ne!(val, 0);
        let x = i % 9;
        let y = (i - x) / 9;
        self.set(x, y, val)
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        self.cells[x][y] = 0;
    }

    pub fn clear_i(&mut self, i: usize) {
        debug_assert!(i < 81);
        let x = i % 9;
        let y = (i - x) / 9;
        self.clear(x, y)
    }

    pub fn count(&self) -> usize {
        (0..81).filter(|i| self.get_i(*i) != 0).count()
    }

    pub fn is_filled(&self) -> bool {
        (0..81).all(|i| self.get_i(i) != 0)
    }
}
