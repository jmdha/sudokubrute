use itertools::Itertools;

use super::Board;

impl Board {
    pub fn is_filled(&self) -> bool {
        (0..81)
            .map(|i| self.get_i(i))
            .filter(|v| v.is_some())
            .count()
            == 9 * 9
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_files() && self.is_valid_ranks() && self.is_valid_boxes()
    }

    pub fn is_valid_files(&self) -> bool {
        (0..9).all(|i| self.is_valid_file(i))
    }

    pub fn is_valid_file(&self, x: usize) -> bool {
        assert!(x < 9);
        (0..9)
            .map(|y| self.get(x, y))
            .filter(|c| c.is_some())
            .duplicates()
            .count()
            == 0
    }

    pub fn is_valid_ranks(&self) -> bool {
        (0..9).all(|i| self.is_valid_rank(i))
    }

    pub fn is_valid_rank(&self, y: usize) -> bool {
        assert!(y < 9);
        (0..9)
            .map(|x| self.get(x, y))
            .filter(|c| c.is_some())
            .duplicates()
            .count()
            == 0
    }

    pub fn is_valid_boxes(&self) -> bool {
        (0..3)
            .cartesian_product(0..3)
            .all(|(x, y)| self.is_valid_box(x, y))
    }

    pub fn is_valid_box(&self, x: usize, y: usize) -> bool {
        assert!(x < 3);
        assert!(y < 3);
        (0..3)
            .cartesian_product(0..3)
            .map(|(x_o, y_o)| self.get(3 * x + x_o, 3 * y + y_o))
            .filter(|c| c.is_some())
            .duplicates()
            .count()
            == 0
    }
}
