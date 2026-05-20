use crate::board::{Board, sample_board};

pub struct App {
    pub running: bool,
    pub board: Board,
    pub selected_row: usize,
    pub selected_col: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            board: sample_board(),
            selected_row: 0,
            selected_col: 0,
        }
    }
}
