use rand::seq::SliceRandom;

use super::solver::{fill_board, has_unique_solution};
use super::{Board, Cell};

#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn clues(&self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 30,
            Difficulty::Hard => 22,
        }
    }
}

pub fn generate_board(difficulty: Difficulty) -> (Board, Board) {
    loop {
        let mut solved_board = Board::new([[Cell::empty(); 9]; 9]);
        fill_board(&mut solved_board);
        solved_board.set_all_values_fixed();
        if let Some(generated_board) = remove_nums_from_board(solved_board, difficulty) {
            return (generated_board, solved_board);
        }
    }
}

fn remove_nums_from_board(solved_board: Board, difficulty: Difficulty) -> Option<Board> {
    let mut generated_board = solved_board.clone();

    let mut indices: Vec<usize> = (0..81).collect();
    indices.shuffle(&mut rand::rng());

    while generated_board.get_num_of_filled_cells() > difficulty.clues() {
        if indices.is_empty() {
            return None;
        }

        let random_index = indices[0];
        indices.remove(0);
        let prev_value = generated_board.get_cell_from_index(random_index).value;

        if prev_value.is_some() {
            generated_board.get_cell_from_index(random_index).value = None;
            generated_board.get_cell_from_index(random_index).fixed = false;
        } else {
            continue;
        }

        if !has_unique_solution(&mut generated_board.clone(), solved_board) {
            generated_board.get_cell_from_index(random_index).value = prev_value;
            generated_board.get_cell_from_index(random_index).fixed = true;
        }
    }
    Some(generated_board)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_board(board: &Board) {
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
    fn test_generate_easy_board() {
        let (puzzle, solved) = generate_board(Difficulty::Easy);

        println!("Puzzle:");
        print_board(&puzzle);
        println!("\nSolved:");
        print_board(&solved);

        assert_eq!(puzzle.get_num_of_filled_cells(), 40);
        assert!(solved.is_complete());
        assert!(has_unique_solution(&mut puzzle.clone(), solved));
    }

    #[test]
    fn test_generate_medium_board() {
        let (puzzle, solved) = generate_board(Difficulty::Medium);

        println!("Puzzle:");
        print_board(&puzzle);
        println!("\nSolved:");
        print_board(&solved);

        assert_eq!(puzzle.get_num_of_filled_cells(), 30);
        assert!(solved.is_complete());
        assert!(has_unique_solution(&mut puzzle.clone(), solved));
    }

    #[test]
    fn test_generate_hard_board() {
        let (puzzle, solved) = generate_board(Difficulty::Hard);

        println!("Puzzle:");
        print_board(&puzzle);
        println!("\nSolved:");
        print_board(&solved);

        assert_eq!(puzzle.get_num_of_filled_cells(), 22);
        assert!(solved.is_complete());
        assert!(has_unique_solution(&mut puzzle.clone(), solved));
    }
}
