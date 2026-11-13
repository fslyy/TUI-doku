use super::Board;
use crate::board_new::check::is_valid_num;
use rand::seq::SliceRandom;

pub fn solve(board: &mut Board) -> Board {
    let mut tried_nums: [[bool; 9]; 81] = [[false; 9]; 81];
    let mut index: usize = 0;

    skip_to_next_unfixed(board, &mut index, true);

    while index < 81 {
        try_valid_nums_for_cell(board, &mut tried_nums, &mut index);
        if index >= 81 {
            break;
        }
        if tried_nums[index].iter().all(|num| *num == true) {
            reset_nums_and_go_back(board, &mut tried_nums, &mut index);
        }
    }
    *board
}

fn skip_to_next_unfixed(board: &mut Board, index: &mut usize, forward: bool) {
    loop {
        if *index >= 81 || !board.get_cell_from_index(*index).fixed {
            break;
        }
        if forward {
            *index += 1;
        } else {
            *index -= 1;
        }
    }
}

fn try_valid_nums_for_cell(board: &mut Board, tried_nums: &mut [[bool; 9]; 81], index: &mut usize) {
    let mut numbers: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    numbers.shuffle(&mut rand::rng());

    for num in numbers {
        let num_index = (num - 1) as usize;
        if is_valid_num(board, num, *index / 9, *index % 9) && !tried_nums[*index][num_index] {
            tried_nums[*index][num_index] = true;
            board.get_cell_from_index(*index).value = Some(num);
            *index += 1;
            skip_to_next_unfixed(board, &mut *index, true);
            break;
        }
        tried_nums[*index][num_index] = true;
    }
}

fn reset_nums_and_go_back(board: &mut Board, tried_nums: &mut [[bool; 9]; 81], index: &mut usize) {
    tried_nums[*index].iter_mut().for_each(|num| *num = false);
    board.get_cell_from_index(*index).value = None;
    *index -= 1;
    skip_to_next_unfixed(board, &mut *index, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_new::{Board, Cell};

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

    #[test]
    fn test_fill_num_completes_board() {
        let mut board = empty_board();

        let result = solve(&mut board);

        for (r, row) in result.cells.iter().enumerate() {
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

        let result = solve(&mut board);

        for (r, row) in result.cells.iter().enumerate() {
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
}
