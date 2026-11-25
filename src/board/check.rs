use super::Board;

pub fn is_valid_box(board: &Board, value: u8, row: usize, col: usize) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board.cells[row + i][col + j].value == Some(value) {
                return false;
            }
        }
    }
    true
}

pub fn is_valid_row(board: &Board, value: u8, row: usize) -> bool {
    board.cells[row]
        .iter()
        .all(|cell| cell.value != Some(value))
}

pub fn is_valid_col(board: &Board, value: u8, col: usize) -> bool {
    board.cells.iter().all(|row| row[col].value != Some(value))
}

pub fn is_valid_num(board: &Board, value: u8, row: usize, col: usize) -> bool {
    is_valid_row(board, value, row)
        && is_valid_col(board, value, col)
        && is_valid_box(board, value, row - row % 3, col - col % 3)
}
