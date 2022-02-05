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
