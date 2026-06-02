use crate::board::{Board, Cell, generate_board, is_valid_num};
use crate::ui::theme::Theme;
use crate::timer::GameTimer;

pub enum Screen {
    MainMenu,
    Game,
}

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub theme: Theme,

    pub board: Board,
    pub solution: Board,

    pub notes_mode: bool,
    pub game_paused: bool,
    pub selected_row: usize,
    pub selected_col: usize,

    pub timer: GameTimer,
    pub win: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            screen: Screen::MainMenu,
            theme: Theme::default()
            ,
            board: Board::new([[Cell::empty(); 9]; 9]),
            solution: Board::new([[Cell::empty(); 9]; 9]),

            notes_mode: false,
            game_paused: false,
            selected_row: 0,
            selected_col: 0,

            timer: GameTimer::new(),
            win: false,
        }
    }

    pub fn start_game(&mut self) {
        let (board, solution) = generate_board();
        self.board = board;
        self.solution = solution;
        self.screen = Screen::Game;
        
        self.selected_row = 0;
        self.selected_col = 0;

        self.timer.reset();
        self.timer.start();
        self.win = false;
    }

    pub fn pause_game(&mut self) {
        if self.game_paused {
            self.timer.resume();
        } else {
            self.timer.pause();
        }
        self.game_paused = !self.game_paused;
    }

    pub fn on_digit(&mut self, c: char) {
        let digit = c.to_digit(10).unwrap() as u8;

        if self.board.cells[self.selected_row][self.selected_col].fixed {
            return;
        }
 
        if self.notes_mode {
            let cell = self.board.get_mut(
                self.selected_row,
                self.selected_col,
            );

            cell.notes[(digit - 1) as usize] =
                !cell.notes[(digit - 1) as usize];
        } else {
            let mut board = self.board;
            board.cells[self.selected_row][self.selected_col].value = None;

            let is_valid = is_valid_num(
                &board,
                digit,
                self.selected_row,
                self.selected_col,
            );

            let cell = self.board.get_mut(
                self.selected_row,
                self.selected_col,
            );

            cell.value = Some(digit);
            cell.is_valid = is_valid;

            if Board::is_complete(&self.board) {
                self.check_win();
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

    pub fn check_win(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                let cell = self.board.get(row, col);
                if cell.value != self.solution.get(row, col).value {
                    return;
                }
            }
        }
        self.timer.pause();
        self.win = true;
    }
}
