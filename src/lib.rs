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
        let next_word = board.compute_next_word_info().0;
        let end = start.elapsed();
        println!("compute next time: {:?}", end);
        next_word
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
    #[test]
    #[ignore]
    fn get_avg_count() {
        let all_answers = CANDITATES.get_canditates();
        let mut average_count = 0;
        all_answers.iter().for_each(|answer| {
            let mut board = ReplFunctionsStruct::reset();
            let first_word: Word = "soare".parse().unwrap();
            let first_status = Word::to_status(&first_word, answer);
            board.filter(&first_word, &first_status);
            average_count += 1;
            loop {
                if board.remaining_canditates.len() == 1 && board.remaining_canditates[0] == *answer
                {
                    break;
                }
                let next_word = ReplFunctionsStruct::next(&mut board);
                let status = Word::to_status(&next_word, answer);
                board.filter(&next_word, &status);
                average_count += 1;
            }
        });
        let average_count = average_count as f64 / all_answers.len() as f64;
        println!("average count: {}", average_count);
    }
}
