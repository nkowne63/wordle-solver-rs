use wordle_solver_rs::commands::ReplCommandHandlers;
use wordle_solver_rs::tactics::position_freq::Board;
use wordle_solver_rs::SHOW_CONSOLE;

fn main() {
    let show_console = SHOW_CONSOLE.clone();
    *show_console.lock().unwrap() = true;
    let mut repl = Board::into_repl();
    repl.run().unwrap();
}
