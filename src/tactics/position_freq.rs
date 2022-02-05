use crate::commands::{ReplCommandHandlers, ReplFunctions};
use crate::enums::{Status, Word};
use crate::tactics::solver::Solver;
use std::{collections::HashMap, time::Instant};

pub struct Board {
    pub remaining_canditates: Vec<Word>,
    input_canditates: Vec<Word>,
}

impl Solver for Board {
    fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Board {
        Board {
            remaining_canditates: canditates,
            input_canditates: inputs,
        }
    }
    fn filter(&mut self, word: &Word, status: &Status) {
        let start = Instant::now();
        let &mut Board {
            ref mut remaining_canditates,
            input_canditates: _,
        } = self;
        let before_len = remaining_canditates.len();
        let remaining: Vec<Word> = remaining_canditates
            .iter()
            .filter(|&answer| status == &Word::to_status(word, answer))
            .cloned()
            .collect();
        let after_len = remaining.len();
        *remaining_canditates = remaining;
        let end = start.elapsed();
        if remaining_canditates.len() > 2 {
            println!(
                "first three: {}, {}, {}",
                remaining_canditates[0].to_string(),
                remaining_canditates[1].to_string(),
                remaining_canditates[2].to_string()
            );
        } else {
            println!(
                "remaining: {:?}",
                remaining_canditates
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>()
            );
        }
        println!("filter: {} -> {}", before_len, after_len);
        println!(
            "gained information: {}",
            (before_len as f64 / after_len as f64).log2()
        );
        println!("filter time: {:?}", end);
    }
    fn next(&self) -> Word {
        let all_start = Instant::now();
        let start = Instant::now();
        let &Board {
            ref remaining_canditates,
            ref input_canditates,
        } = self;

        todo!()
    }
}

impl ReplFunctions for Board {}
impl ReplCommandHandlers for Board {}

#[cfg(test)]
mod bench {
    use super::*;
    use crate::enums::Word;
    use crate::tactics::solver::Solver;
    use crate::CANDITATES;
    #[test]
    #[ignore]
    fn get_maximum() {
        let mut board = Board::reset();
        <Board as ReplFunctions>::next(&mut board);
    }
    #[test]
    #[ignore]
    fn get_avg_count() {
        let best_first = "soare";
        let all_answers = CANDITATES.get_canditates();
        let mut average_count = 0;
        all_answers.iter().for_each(|answer| {
            let mut board = Board::reset();
            let first_word: Word = best_first.parse().unwrap();
            let first_status = Word::to_status(&first_word, answer);
            board.filter(&first_word, &first_status);
            average_count += 1;
            loop {
                if board.remaining_canditates.len() == 1 && board.remaining_canditates[0] == *answer
                {
                    average_count += 1;
                    break;
                }
                let next_word = <Board as ReplFunctions>::next(&mut board);
                let status = Word::to_status(&next_word, answer);
                board.filter(&next_word, &status);
                average_count += 1;
            }
        });
        let average_count = average_count as f64 / all_answers.len() as f64;
        println!("average count: {}", average_count);
    }
}
