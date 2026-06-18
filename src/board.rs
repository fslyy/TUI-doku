use serde::{Deserialize, Serialize};
use sudokugen::{BoardSize, Puzzle};

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

impl From<&sudokugen::Board> for Board {
    fn from(src: &sudokugen::Board) -> Self {
        let mut cells = [[Cell::empty(); 9]; 9];

        for row in 0..9 {
            for col in 0..9 {
                let value = src.get(&src.cell_at(row, col));

                cells[row][col] = match value {
                    Some(v) => Cell::fixed(v),
                    None => Cell::empty(),
                };
            }
        }

        Board::new(cells)
    }
}

impl From<&Board> for sudokugen::Board {
    fn from(src: &Board) -> Self {
        let mut board = sudokugen::Board::new(BoardSize::NineByNine);

        for row in 0..9 {
            for col in 0..9 {
                if let Some(value) = src.cells[row][col].value {
                    let cell = board.cell_at(row, col);
                    board.set(&cell, value);
                }
            }
        }

        board
    }
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

    pub fn is_complete(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[row][col].value.is_none() {
                    return false;
                }
            }
        }
        true
    }
}

pub fn generate_board() -> (Board, Board) {
    let puzzle = Puzzle::generate(BoardSize::NineByNine);

    let board = Board::from(puzzle.board());
    let solution = Board::from(puzzle.solution());

    (board, solution)
}

pub fn solve_board(board: &Board) -> Board {
    let mut puzzle = board.clone();

    for row in 0..9 {
        for col in 0..9 {
            let cell = &mut puzzle.cells[row][col];
            if cell.fixed == false {
                cell.value = None;
            }
        }
    }

    let mut solver_board = sudokugen::Board::from(&puzzle);
    solver_board.solve().unwrap();
    Board::from(&solver_board)
}

// BOARD LOGIC
pub fn is_valid_box(board: &Board, value: u8, row: usize, col: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if (board.cells[row + i][col + j].value == Some(value)) {
                return false;
            }
        }
    }
    return true;
}

pub fn is_valid_row(board: &Board, value: u8, row: usize) -> bool {
    for col in 0..9 {
        if board.cells[row][col].value == Some(value) {
            return false;
        }
    }
    return true;
}

pub fn is_valid_col(board: &Board, value: u8, col: usize) -> bool {
    for row in 0..9 {
        if board.cells[row][col].value == Some(value) {
            return false;
        }
    }
    return true;
}

pub fn is_valid_num(board: &Board, value: u8, row: usize, col: usize) -> bool {
    is_valid_row(board, value, row)
        && is_valid_col(board, value, col)
        && is_valid_box(board, value, row - row % 3, col - col % 3)
}

#[test]
fn test_generate_board() {
    let (board, solution) = generate_board();
    assert_eq!(board.cells.len(), 9);
    assert_eq!(solution.cells.len(), 9);
    // for row in 0..9 {
    //    for col in 0..9 {
    //         dbg!(row, col, board.get(row, col).value, solution.get(row, col).value);
    //     }
    // }
}
