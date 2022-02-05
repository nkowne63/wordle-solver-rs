use crate::{
    enums::{Status, Word},
    tactics::solver::Solver,
    CANDITATES,
};
use repl_rs::{Command, Convert, Error as ReplError, Parameter, Repl, Value};
use std::collections::HashMap;

pub struct ReplContext<T: Solver> {
    board: T,
}

pub trait ReplFunctions: Solver + Sized {
    fn reset() -> Self {
        Self::new(CANDITATES.get_canditates(), CANDITATES.get_all_words())
    }
    fn filter(word: Word, status: Status, board: &mut Self) {
        board.filter(&word, &status);
    }
    fn next(board: &mut Self) -> Word {
        board.next()
    }
}

pub trait ReplCommandHandlers: ReplFunctions {
    fn reset_handler(
        _args: HashMap<String, Value>,
        context: &mut ReplContext<Self>,
    ) -> Result<Option<String>, ReplError> {
        context.board = Self::reset();
        Ok(None)
    }
    fn filter_handler(
        args: HashMap<String, Value>,
        context: &mut ReplContext<Self>,
    ) -> Result<Option<String>, ReplError> {
        let word_string: String = args.get("word").unwrap().convert()?;
        let status_string: String = args.get("status").unwrap().convert()?;
        let word: Word = word_string.parse().unwrap();
        let status: Status = status_string.parse().unwrap();
        let board = &mut context.board;
        <Self as ReplFunctions>::filter(word, status, board);
        Ok(None)
    }
    fn next_handler(
        _args: HashMap<String, Value>,
        context: &mut ReplContext<Self>,
    ) -> Result<Option<String>, ReplError> {
        let board = &mut context.board;
        let word = <Self as ReplFunctions>::next(board);
        Ok(Some(word.to_string()))
    }
    fn into_repl() -> Repl<ReplContext<Self>, ReplError> {
        let reset_command =
            Command::new("reset", Self::reset_handler).with_help("Reset wordle solver state");
        let next_command =
            Command::new("next", Self::next_handler).with_help("Get next wordle solution");
        let filter_command = Command::new("filter", Self::filter_handler)
            .with_help("Filter wordle canditates of solutions")
            .with_parameter(Parameter::new("word").set_required(true).unwrap())
            .unwrap()
            .with_parameter(Parameter::new("status").set_required(true).unwrap())
            .unwrap();
        let ctx = ReplContext {
            board: Self::reset(),
        };
        Repl::new(ctx)
            .with_name("wordle-solver")
            .with_version("v0.0.1")
            .with_description("Wordle solver")
            .add_command(reset_command)
            .add_command(next_command)
            .add_command(filter_command)
    }
}
