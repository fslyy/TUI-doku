use super::Board;
use rand::seq::SliceRandom;

pub fn fill_board(board: &mut Board) -> bool {
    let mut bitmask_row: [u32; 9] = [0b0000000; 9];
    let mut bitmask_col: [u32; 9] = [0b0000000; 9];
    let mut bitmask_sqr: [u32; 9] = [0b0000000; 9];
    solve_logic(
        board,
        0,
        0,
        true,
        false,
        &mut bitmask_row,
        &mut bitmask_col,
        &mut bitmask_sqr,
    ) // is this truly random ?
}

pub fn solve_board(board: &mut Board) -> bool {
    let mut bitmask_row: [u32; 9] = [0b0000000; 9];
    let mut bitmask_col: [u32; 9] = [0b0000000; 9];
    let mut bitmask_sqr: [u32; 9] = [0b0000000; 9];
    initialize_bitmasks(board, &mut bitmask_row, &mut bitmask_col, &mut bitmask_sqr);
    solve_logic(
        board,
        0,
        0,
        false,
        false,
        &mut bitmask_row,
        &mut bitmask_col,
        &mut bitmask_sqr,
    )
}

pub fn has_unique_solution(board: &mut Board, solution: Board) -> bool {
    let mut tried_solution = board.clone();
    let mut bitmask_row: [u32; 9] = [0b0000000; 9];
    let mut bitmask_col: [u32; 9] = [0b0000000; 9];
    let mut bitmask_sqr: [u32; 9] = [0b0000000; 9];
    initialize_bitmasks(board, &mut bitmask_row, &mut bitmask_col, &mut bitmask_sqr);
    solve_logic(
        &mut tried_solution,
        0,
        0,
        false,
        true,
        &mut bitmask_row,
        &mut bitmask_col,
        &mut bitmask_sqr,
    );

    tried_solution.get_cells_as_str() == solution.get_cells_as_str()
}

fn initialize_bitmasks(
    board: &mut Board,
    bitmask_row: &mut [u32; 9],
    bitmask_col: &mut [u32; 9],
    bitmask_sqr: &mut [u32; 9],
) {
    for row in 0..9 {
        for col in 0..9 {
            if board.cells[row][col].value.is_some() {
                let value = board.cells[row][col].value.unwrap() as u32;
                bitmask_row[row] |= 1 << value;
                bitmask_col[col] |= 1 << value;
                bitmask_sqr[(row / 3) * 3 + (col / 3)] |= 1 << value;
            }
        }
    }
}

fn solve_logic(
    board: &mut Board,
    row: usize,
    col: usize,
    random: bool,
    reverse: bool,
    bitmask_row: &mut [u32; 9],
    bitmask_col: &mut [u32; 9],
    bitmask_sqr: &mut [u32; 9],
) -> bool {
    let n = 9;

    let mut numbers = [0; 9];

    if reverse {
        numbers = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    } else {
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    }
    if random {
        numbers.shuffle(&mut rand::rng());
    }

    if row == n - 1 && col == n {
        return true;
    }

    if col == n {
        return solve_logic(
            board,
            row + 1,
            0,
            random,
            reverse,
            bitmask_row,
            bitmask_col,
            bitmask_sqr,
        );
    }

    if board.cells[row][col].value.is_some() {
        return solve_logic(
            board,
            row,
            col + 1,
            random,
            reverse,
            bitmask_row,
            bitmask_col,
            bitmask_sqr,
        );
    }

    let sqr = (row / 3) * 3 + (col / 3);

    for num in numbers {
        if (bitmask_col[col] & 1 << num == 0)
            && (bitmask_row[row] & 1 << num == 0)
            && (bitmask_sqr[sqr] & 1 << num == 0)
        {
            board.cells[row][col].value = Some(num);
            bitmask_row[row] |= 1 << num;
            bitmask_col[col] |= 1 << num;
            bitmask_sqr[sqr] |= 1 << num;
            if solve_logic(
                board,
                row,
                col + 1,
                random,
                reverse,
                bitmask_row,
                bitmask_col,
                bitmask_sqr,
            ) {
                return true;
            }
            board.cells[row][col].value = None;
            bitmask_row[row] &= !(1 << num);
            bitmask_col[col] &= !(1 << num);
            bitmask_sqr[sqr] &= !(1 << num);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{Board, Cell};

    fn empty_board() -> Board {
        Board::new([[Cell::empty(); 9]; 9])
    }

    fn partial_board() -> Board {
        let e = Cell::empty;
        let f = Cell::fixed;
        Board::new([
            [f(5), f(3), e(), e(), f(7), e(), e(), e(), e()],
            [f(6), e(), e(), f(1), f(9), f(5), e(), e(), e()],
            [e(), f(9), f(8), e(), e(), e(), e(), f(6), e()],
            [f(8), e(), e(), e(), f(6), e(), e(), e(), f(3)],
            [f(4), e(), e(), f(8), e(), f(3), e(), e(), f(1)],
            [f(7), e(), e(), e(), f(2), e(), e(), e(), f(6)],
            [e(), f(6), e(), e(), e(), e(), f(2), f(8), e()],
            [e(), e(), e(), f(4), f(1), f(9), e(), e(), f(5)],
            [e(), e(), e(), e(), f(8), e(), e(), f(7), f(9)],
        ])
    }

    fn multi_solution_board() -> Board {
        let e = Cell::empty;
        let f = Cell::fixed;
        Board::new([
            [f(1), f(2), f(3), f(4), f(5), f(6), f(7), f(8), f(9)],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
            [e(), e(), e(), e(), e(), e(), e(), e(), e()],
        ])
    }

    #[test]
    fn test_fill_num_completes_board() {
        let mut board = empty_board();

        fill_board(&mut board);

        for (r, row) in board.cells.iter().enumerate() {
            if r % 3 == 0 && r != 0 {
                println!("------+-------+------");
            }
            for (c, cell) in row.iter().enumerate() {
                if c % 3 == 0 && c != 0 {
                    print!(" | ");
                }
                match cell.value {
                    Some(v) => print!("{}", v),
                    None => print!("."),
                }
                if c < 8 {
                    print!(" ");
                }
            }
            println!();
        }
    }

    #[test]
    fn test_solve_partial_board() {
        let mut board = partial_board();

        solve_board(&mut board);

        for (r, row) in board.cells.iter().enumerate() {
            if r % 3 == 0 && r != 0 {
                println!("------+-------+------");
            }
            for (c, cell) in row.iter().enumerate() {
                if c % 3 == 0 && c != 0 {
                    print!(" | ");
                }
                match cell.value {
                    Some(v) => print!("{}", v),
                    None => print!("."),
                }
                if c < 8 {
                    print!(" ");
                }
            }
            println!();
        }
    }

    #[test]
    fn test_unique_solution_returns_true() {
        let puzzle = partial_board();
        let mut solution = puzzle.clone();
        solve_board(&mut solution);
        assert!(has_unique_solution(&mut puzzle.clone(), solution));
    }

    #[test]
    fn test_multi_solution_returns_false() {
        let puzzle = multi_solution_board();
        let mut solution = puzzle.clone();
        solve_board(&mut solution);
        assert!(!has_unique_solution(&mut puzzle.clone(), solution));
    }
}
