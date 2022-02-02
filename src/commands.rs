use repl_rs::{Command, Convert, Error as ReplError, Parameter, Repl, Value};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Debug)]
enum GenericError<CommandError: Debug> {
    CommandError(CommandError),
    ReplError(ReplError),
}

impl<CommandError: Debug> From<ReplError> for GenericError<CommandError> {
    fn from(error: ReplError) -> Self {
        GenericError::ReplError(error)
    }
}

impl<CommandError: Debug> Display for GenericError<CommandError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericError::CommandError(error) => write!(f, "Command error: {:#?}", error),
            GenericError::ReplError(error) => write!(f, "Repl error: {}", error),
        }
    }
}

pub struct ReplStruct<Context, CommandError: Debug> {
    repl: Repl<Context, GenericError<CommandError>>,
}

impl<Context, CommandError: Debug> ReplStruct<Context, CommandError> {
    pub fn run(&mut self) {
        self.repl.run().unwrap()
    }
}

pub struct ReplCommands<Context, Error: Debug> {
    reset: Command<Context, GenericError<Error>>,
    filter: Command<Context, GenericError<Error>>,
    next: Command<Context, GenericError<Error>>,
}

impl<Context, CommandError: Debug> ReplCommands<Context, CommandError> {
    pub fn into_repl(self, ctx: Context) -> ReplStruct<Context, CommandError> {
        let repl = Repl::new(ctx)
            .with_name("wordle-solver")
            .with_version("v0.0.1")
            .with_description("Wordle solver")
            .add_command(self.reset)
            .add_command(self.filter)
            .add_command(self.next);
        ReplStruct { repl }
    }
}

type CommandHandlerFunc<Context, CommandError> =
    fn(
        args: HashMap<String, Value>,
        context: &mut Context,
    ) -> Result<Option<String>, GenericError<CommandError>>;

pub struct ReplCommandHandlers<Context, CommandError: Debug> {
    reset: CommandHandlerFunc<Context, CommandError>,
    filter: CommandHandlerFunc<Context, CommandError>,
    next: CommandHandlerFunc<Context, CommandError>,
}

impl<Context, CommandError: Debug> ReplCommandHandlers<Context, CommandError> {
    pub fn into_commands(self) -> ReplCommands<Context, CommandError> {
        let reset = Command::new("reset", self.reset).with_help("Reset wordle solver state");
        let next = Command::new("next", self.next).with_help("Get next wordle solution");
        let filter = Command::new("filter", self.filter)
            .with_help("Filter wordle canditates of solutions")
            .with_parameter(Parameter::new("word").set_required(true).unwrap())
            .unwrap()
            .with_parameter(Parameter::new("state").set_required(true).unwrap())
            .unwrap();
        ReplCommands {
            reset,
            filter,
            next,
        }
    }
}

#[derive(Debug)]
pub enum ArgParseError<WordParseError: Debug, StatusParseError: Debug> {
    WordParseError(WordParseError),
    StatusParseError(StatusParseError),
}

pub struct BoardContext<Board> {
    board: Board,
}

impl<Board> BoardContext<Board> {
    fn replace(&mut self, board: Board) {
        self.board = board;
    }
}

pub trait ReplFunctions<Board, Word, Status>: Sized
where
    Word: TryFrom<String> + ToString,
    Status: TryFrom<String> + ToString,
    Word::Error: Debug,
    Status::Error: Debug,
{
    fn reset() -> Board;
    fn filter(board: &mut Board, word: &Word, status: &Status);
    fn next(board: &Board) -> Word;
    fn into_handlers(
    ) -> ReplCommandHandlers<BoardContext<Board>, ArgParseError<Word::Error, Status::Error>> {
        let reset: CommandHandlerFunc<
            BoardContext<Board>,
            ArgParseError<Word::Error, Status::Error>,
        > = |_args, ctx| {
            let board = Self::reset();
            ctx.replace(board);
            Ok(Some("solver has reset the state".to_string()))
        };
        let filter: CommandHandlerFunc<
            BoardContext<Board>,
            ArgParseError<Word::Error, Status::Error>,
        > = |args, ctx| {
            let word_string: String = args.get("word").unwrap().convert().unwrap();
            let status_string: String = args.get("status").unwrap().convert().unwrap();
            let word_parse_result: Result<Word, Word::Error> = word_string.try_into();
            let status_parse_result: Result<Status, Status::Error> = status_string.try_into();
            match (word_parse_result, status_parse_result) {
                (Ok(word), Ok(status)) => {
                    Self::filter(&mut ctx.board, &word, &status);
                    Ok(Some(format!(
                        "solver has filtered the wordle candidates for word: {} and status: {}",
                        word.to_string(),
                        status.to_string()
                    )))
                }
                (Err(word_error), _) => Err(GenericError::CommandError(
                    ArgParseError::WordParseError(word_error),
                )),
                (_, Err(status_error)) => Err(GenericError::CommandError(
                    ArgParseError::StatusParseError(status_error),
                )),
            }
        };
        let next: CommandHandlerFunc<
            BoardContext<Board>,
            ArgParseError<Word::Error, Status::Error>,
        > = |_args, ctx| {
            let next_word = Self::next(&ctx.board);
            Ok(Some(format!("next word is: {}", next_word.to_string())))
        };
        ReplCommandHandlers {
            reset,
            filter,
            next,
        }
    }
}

#[cfg(test)]
mod test {
    #[allow(clippy::eq_op)]
    #[test]
    fn sample() {
        assert_eq!(2 + 2, 4);
    }
}
