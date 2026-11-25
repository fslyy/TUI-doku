use crate::board::{
    Board, Cell, generator::Difficulty, generator::generate_board, solver::solve_board,
};
use crate::save::game::{GameSave, clear_game_state, load_game_state, save_game_state};
use crate::save::stats::update_and_save_stats;
use crate::timer::GameTimer;
use crate::ui::theme::Theme;

pub enum Screen {
    MainMenu,
    Stats,
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
            theme: Theme::default(),
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

    pub fn save_game(&mut self) {
        self.timer.pause();
        match save_game_state(&GameSave::new(
            self.board,
            self.selected_row,
            self.selected_col,
            self.timer.elapsed().as_secs(),
        )) {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err.to_string());
            }
        }
    }

    pub fn clear_saved_game(&mut self) {
        match clear_game_state() {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err.to_string());
            }
        }
    }

    pub fn load_game(&mut self) {
        match load_game_state() {
            Ok(save) => {
                self.board = save.board;
                let mut solution = save.board;
                solve_board(&mut solution);
                self.solution = solution;

                self.selected_col = save.selected_col;
                self.selected_row = save.selected_row;

                self.timer.start(Some(save.elapsed_seconds));

                self.win = false;
                self.screen = Screen::Game;
            }

            Err(err) => {
                self.start_game();
                println!("{:?}", err.to_string());
            }
        }
    }

    pub fn start_game(&mut self) {
        // TO-DO: extra ui screen for selecting difficulty -> then maybe also different stats depending on difficulty
        let (board, solution) = generate_board(Difficulty::Medium);
        self.board = board;
        self.solution = solution;
        self.screen = Screen::Game;

        self.selected_row = 0;
        self.selected_col = 0;

        self.timer.reset();
        self.timer.start(None);
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
        let _ = update_and_save_stats(self.timer.elapsed().as_secs());
    }
}
