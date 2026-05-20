#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub value: Option<u8>,
    pub notes: [bool; 9],
    pub fixed: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub cells: [[Cell;9];9]
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            value: None,
            notes: [false; 9],
            fixed: false,
        }
    }
    pub fn fixed(value: u8) -> Self {
        Self {
            value: Some(value),
            notes: [false; 9],
            fixed: true,
        }
    }
    pub fn with_value(value: u8) -> Self {
        Self {
            value: Some(value),
            notes: [false; 9],
            fixed: false,
        }
    }
}

impl Board {
    pub fn new(cells: [[Cell; 9]; 9]) -> Self {
        Self { cells }
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.cells[row][col]
    }
}

pub fn sample_board() -> Board {
    Board::new([
        [
            Cell::fixed(5),
            Cell::fixed(3),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(7),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
        ],
        [
            Cell::fixed(6),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(1),
            Cell::fixed(9),
            Cell::fixed(5),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
        ],
        [
            Cell::empty(),
            Cell::fixed(9),
            Cell::fixed(8),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(6),
            Cell::empty(),
        ],
        [
            Cell::fixed(8),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(6),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(3),
        ],
        [
            Cell::fixed(4),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(8),
            Cell::empty(),
            Cell::fixed(3),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(1),
        ],
        [
            Cell::fixed(7),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(2),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(6),
        ],
        [
            Cell::empty(),
            Cell::fixed(6),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(2),
            Cell::fixed(8),
            Cell::empty(),
        ],
        [
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(4),
            Cell::fixed(1),
            Cell::fixed(9),
            Cell::empty(),
            Cell::fixed(5),
            Cell::empty(),
        ],
        [
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(8),
            Cell::empty(),
            Cell::empty(),
            Cell::fixed(7),
            Cell::fixed(9),
        ],
    ])
}
