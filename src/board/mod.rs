pub mod construction;
pub mod debug;
pub mod validity;

#[derive(Clone)]
pub struct Board {
    cells: [[u8; 9]; 9],
    valid_ranks: [u16; 9],
    valid_files: [u16; 9],
    valid_boxes: [[u16; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        let cells = [[0; 9]; 9];
        let mut valid_ranks = [0; 9];
        let mut valid_files = [0; 9];

        for i in 0..9 {
            for v in 1..=9 {
                valid_ranks[i] |= 1 << v;
                valid_files[i] |= 1 << v;
            }
        }

        let mut valid_boxes = [[0; 3]; 3];

        for x in 0..3 {
            for y in 0..3 {
                for v in 1..=9 {
                    valid_boxes[x][y] |= 1 << v;
                }
            }
        }

        Self {
            cells,
            valid_ranks,
            valid_files,
            valid_boxes,
        }
    }
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
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
        self.set_unchecked(x, y, val);
        true
    }

    pub fn set_i(&mut self, i: usize, val: u8) -> bool {
        debug_assert!(i < 81);
        debug_assert_ne!(val, 0);
        let x = i % 9;
        let y = (i - x) / 9;
        self.set(x, y, val)
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, val: u8) {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        debug_assert_ne!(val, 0);
        debug_assert!(self.is_valid(x, y, val));
        self.cells[x][y] = val;
        debug_assert!(self.valid_ranks[y] & (1 << val) != 0);
        debug_assert!(self.valid_files[x] & (1 << val) != 0);
        debug_assert!(self.get_box(x, y) & (1 << val) != 0);
        self.valid_ranks[y] &= !(1 << val);
        self.valid_files[x] &= !(1 << val);
        *self.get_box_mut(x, y) &= !(1 << val);
    }

    pub fn set_unchecked_i(&mut self, i: usize, val: u8) {
        debug_assert!(i < 81);
        debug_assert_ne!(val, 0);
        let x = i % 9;
        let y = (i - x) / 9;
        self.set_unchecked(x, y, val)
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        let p_val = self.cells[x][y];
        debug_assert!(self.valid_ranks[y] & (1 << p_val) == 0);
        debug_assert!(self.valid_files[x] & (1 << p_val) == 0);
        debug_assert!(self.get_box(x, y) & (1 << p_val) == 0);
        self.valid_ranks[y] |= 1 << p_val;
        self.valid_files[x] |= 1 << p_val;
        *self.get_box_mut(x, y) |= 1 << p_val;
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
        self.count() == 81
    }

    pub fn candidates(&self, x: usize, y: usize) -> u16 {
        self.valid_ranks[y] & self.valid_files[x] & self.get_box(x, y)
    }

    pub fn candidates_i(&self, i: usize) -> u16 {
        debug_assert!(i < 81);
        let x = i % 9;
        let y = (i - x) / 9;
        self.candidates(x, y)
    }

    fn get_box(&self, x: usize, y: usize) -> &u16 {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        let x = x / 3;
        let y = y / 3;
        &self.valid_boxes[x][y]
    }

    fn get_box_mut(&mut self, x: usize, y: usize) -> &mut u16 {
        debug_assert!(x < 9);
        debug_assert!(y < 9);
        let x = x / 3;
        let y = y / 3;
        &mut self.valid_boxes[x][y]
    }
}
