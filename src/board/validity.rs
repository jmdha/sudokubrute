use itertools::Itertools;

use super::Board;

impl Board {
    pub(super) fn is_valid(&self, x: usize, y: usize, val: u8) -> bool {
        assert!(val != 0);
        self.is_valid_rank(y, val) && self.is_valid_file(x, val) && self.is_valid_box(x, y, val)
    }

    pub fn is_valid_rank(&self, y: usize, val: u8) -> bool {
        assert!(val != 0);
        (0..9).all(|x| self.get(x, y) != val)
    }

    fn is_valid_file(&self, x: usize, val: u8) -> bool {
        assert!(val != 0);
        (0..9).all(|y| self.get(x, y) != val)
    }

    fn is_valid_box(&self, x: usize, y: usize, val: u8) -> bool {
        assert!(val != 0);
        let x_offset = x - x % 3;
        let y_offset = y - y % 3;
        (0..3)
            .cartesian_product(0..3)
            .all(|(x, y)| self.get(x + x_offset, y + y_offset) != val)
    }
}
