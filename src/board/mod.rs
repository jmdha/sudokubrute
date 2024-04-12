pub mod construction;
pub mod debug;
pub mod validity;

pub type Cell = Option<u8>;

#[derive(Clone, Default)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }

    pub fn get_i(&self, i: usize) -> Cell {
        let x = i % 9;
        let y = (i - x) / 9;
        self.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, val: Cell) {
        self.cells[x][y] = val;
    }

    pub fn set_i(&mut self, i: usize, val: Cell) {
        let x = i % 9;
        let y = (i - x) / 9;
        self.set(x, y, val)
    }
}
