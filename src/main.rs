use wordle_solver_rs::commands::ReplCommandHandlers;
use wordle_solver_rs::tactics::position_freq::Board;

fn main() {
    let mut repl = Board::into_repl();
    repl.run().unwrap();
}
