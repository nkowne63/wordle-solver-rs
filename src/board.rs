use crate::enums::{Status, Word};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordPair(Word, Word);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordStatusPair(Word, Status);

struct Board {
    status_board: HashMap<WordPair, Status>,
    remaining_canditates: Vec<Word>,
    input_canditates: Vec<Word>,
    word_color_grouping: HashMap<WordStatusPair, u32>,
    word_avg_info: HashMap<Word, f64>,
}

impl Board {
    fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Board {
        let status_board = HashMap::new();
        let word_color_grouping = HashMap::new();
        let word_avg_info = HashMap::new();
        Board {
            status_board,
            remaining_canditates: canditates,
            input_canditates: inputs,
            word_color_grouping,
            word_avg_info,
        }
    }
    fn prepare_state(&mut self) {
        let &mut Board {
            // given
            ref mut remaining_canditates,
            ref mut input_canditates,
            ref mut status_board,
            ref mut word_color_grouping,
            ref mut word_avg_info,
        } = self;
        let len_of_input_canditates = input_canditates.len() as u32;
        input_canditates.iter().for_each(|word| {
            remaining_canditates.iter().for_each(|answer| {
                let status = Word::to_status(word, answer);
                status_board.insert(WordPair(*word, *answer), status);
                word_color_grouping.insert(
                    WordStatusPair(*word, status),
                    *word_color_grouping
                        .get(&WordStatusPair(*word, status))
                        .unwrap_or(&0)
                        + 1,
                );
            });
            let mut avg_info = 0f64;
            Status::get_status_iter().for_each(|status| {
                let count = *word_color_grouping
                    .get(&WordStatusPair(*word, status))
                    .unwrap_or(&0);
                let probablity = count as f64 / len_of_input_canditates as f64;
                avg_info += -probablity * probablity.log2();
            });
            word_avg_info.insert(*word, avg_info);
        });
    }
}
