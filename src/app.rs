use crate::board::{Board, generate_board, is_valid_num};

pub struct App {
    pub running: bool,
    pub board: Board,
    pub solution: Board,
    pub notes: bool,
    pub selected_row: usize,
    pub selected_col: usize,
}

impl App {
    pub fn new() -> Self {
        let (board, solution) = generate_board();

        Self {
            running: true,
            board,
            solution,
            notes: false,
            selected_row: 0,
            selected_col: 0,
        }
    }

    pub fn on_digit(&mut self, c: char) {
        let digit = c.to_digit(10).unwrap() as u8;
        let is_valid = is_valid_num(&self.board, digit, self.selected_row, self.selected_col);

        let cell = Board::get_mut(
            &mut self.board,
            self.selected_row,
            self.selected_col,
        );
        
        if !cell.fixed {
            if self.notes {
                cell.notes[(digit - 1) as usize] = !cell.notes[(digit - 1) as usize];
            } else {
                cell.value = Some(digit);
                cell.is_valid = is_valid;
                if Board::is_complete(&self.board) {
                    self.check_win();
                }
            }
        }
    }

    pub fn on_backspace(&mut self) {
        let cell = Board::get_mut(
            &mut self.board,
            self.selected_row,
            self.selected_col,
        );

        if !cell.fixed {
            cell.value = None;
            cell.is_valid = true;
        }
    }

    pub fn check_win(&self) {
        for row in 0..9 {
            for col in 0..9 {
                let cell = self.board.get(row, col);
                if cell.value != self.solution.get(row, col).value {
                    return;
                }
            }
        }
        dbg!("You win!");  
    }
}
