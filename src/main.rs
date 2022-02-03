use wordle_solver_rs::commands::ReplCommandHandlers;
use wordle_solver_rs::ReplFunctionsStruct;

fn main() {
    let mut repl = ReplFunctionsStruct::into_repl();
    repl.run().unwrap();
}
