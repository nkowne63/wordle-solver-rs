use crate::commands::ReplCommandHandlers;
use crate::{set_show_console, tactics};

pub fn avg_info_max() {
    set_show_console(true);
    let mut repl = tactics::avg_info_max::Board::into_repl();
    repl.run().unwrap();
}

pub fn position_freq() {
    set_show_console(true);
    let mut repl = tactics::position_freq::Board::into_repl();
    repl.run().unwrap();
}

pub mod leko_competition {
    use crate::{leko_competition::interface::LekoRepl, tactics::solver::Solver, CANDITATES};

    use super::*;

    pub fn position_freq() {
        set_show_console(false);
        let mut board = tactics::position_freq::Board::new(
            CANDITATES.get_canditates(),
            CANDITATES.get_all_words(),
        );
        board.run_repl();
    }

    pub fn avg_info_max() {
        set_show_console(false);
        let mut board = tactics::avg_info_max::Board::new(
            CANDITATES.get_canditates(),
            CANDITATES.get_all_words(),
        );
        board.run_repl();
    }
}
