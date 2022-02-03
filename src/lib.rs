mod board;
pub mod commands;
mod enums;
mod words;

use std::time::Instant;

use board::Board;
use commands::{ReplCommandHandlers, ReplFunctions};
use enums::{Status, Word};
use once_cell::sync::Lazy;
use words::Canditates;

static CANDITATES: Lazy<Canditates> = Lazy::new(|| Canditates {});

pub struct ReplFunctionsStruct {}

impl ReplFunctions for ReplFunctionsStruct {
    fn reset() -> Board {
        Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words())
    }

    fn filter(word: Word, status: Status, board: &mut Board) {
        board.filter(&word, &status);
    }

    fn next(board: &mut Board) -> Word {
        let start = Instant::now();
        board.compute_board();
        board.compute_info();
        let end = start.elapsed();
        println!("compute next time: {:?}", end);
        board.compute_next_word_info().0
    }
}

impl ReplCommandHandlers for ReplFunctionsStruct {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn get_maximum() {
        let mut board = ReplFunctionsStruct::reset();
        ReplFunctionsStruct::next(&mut board);
    }
}
