use super::Board;
use rand::seq::SliceRandom;
use crate::board_new::check::is_valid_num;

pub fn solve(board: &mut Board) -> Board {
    let mut tried_nums: [[bool; 9]; 81] = [[false; 9]; 81];
    let mut index: usize = 0;

    while index < 81 {
        if !board.get_cell_from_index(index).fixed {
            let mut numbers: Vec<u8> = (1..=9).collect();
            numbers.shuffle(&mut rand::rng());
            for num in numbers {
                if is_valid_num(board, num, index / 9, index % 9) && !tried_nums[index][(num-1) as usize] {
                    tried_nums[index][(num-1) as usize] = true;
                    board.get_cell_from_index(index).value = Some(num);
                    loop {
                        index += 1;
                        if index >= 81 || !board.get_cell_from_index(index).fixed {
                            break;
                        }
                    }
                    break;
                }
                tried_nums[index][(num-1) as usize] = true;
            }
            if index >= 81 {
                break;
            }
            if tried_nums[index].iter().all(|num| *num == true) {
                tried_nums[index].iter_mut().for_each(|num|*num = false);
                board.get_cell_from_index(index).value = None;
                loop {
                    index -= 1;
                    if !board.get_cell_from_index(index).fixed {
                        break;
                    }
                }
            }
        }
    }
    *board
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_new::{Board, Cell};

    fn empty_board() -> Board {
        Board::new([[Cell::empty(); 9]; 9])
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
                if c < 8 { print!(" "); }
            }
            println!();
        }
    }
}