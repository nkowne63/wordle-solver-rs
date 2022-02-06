use crate::{
    set_show_console,
    tactics::{avg_info_max, position_freq},
};

#[cfg(feature = "local")]
pub mod local {
    use super::*;
    use crate::commands::{ReplCommandHandlers, ReplFunctions};

    impl ReplFunctions for avg_info_max::Board {}
    impl ReplCommandHandlers for avg_info_max::Board {}
    impl ReplFunctions for position_freq::Board {}
    impl ReplCommandHandlers for position_freq::Board {}

    pub fn avg_info_max() {
        set_show_console(true);
        let mut repl = avg_info_max::Board::into_repl();
        repl.run().unwrap();
    }

    pub fn position_freq() {
        set_show_console(true);
        let mut repl = position_freq::Board::into_repl();
        repl.run().unwrap();
    }
}

pub mod leko_competition {
    use crate::{leko_competition::interface::LekoRepl, tactics::solver::Solver, CANDITATES};

    use super::*;

    pub fn position_freq() {
        set_show_console(false);
        let mut board =
            position_freq::Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words());
        board.run_repl();
    }

    pub fn avg_info_max() {
        set_show_console(false);
        let mut board =
            avg_info_max::Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words());
        board.run_repl();
    }
}
