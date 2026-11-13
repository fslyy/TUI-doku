pub mod check;
pub mod generator;
pub mod solver;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Cell {
    pub value: Option<u8>,
    pub notes: [bool; 9],
    pub fixed: bool,
    pub is_valid: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            value: None,
            notes: [false; 9],
            fixed: false,
            is_valid: true,
        }
    }
    pub fn fixed(value: u8) -> Self {
        Self {
            value: Some(value),
            notes: [false; 9],
            fixed: true,
            is_valid: true,
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

    pub fn get_cell_from_index(&mut self, index: usize) -> &mut Cell {
        let row = index / 9;
        let col = index % 9;
        &mut self.cells[row][col]
    }

    pub fn is_complete(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.value.is_some())
    }

    pub fn is_complete_num(&self, value: u8) -> bool {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| cell.value == Some(value))
            .count()
            == 9
    }
}
